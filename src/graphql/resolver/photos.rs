use juniper::{FieldResult};
use diesel::{Queryable};
use crate::diesel::RunQueryDsl;
use crate::diesel::prelude::*;
use crate::schema::photos::dsl::{ photos, marker };
use crate::schema::destination_marker_stats_copy::dsl::{ destination_marker_stats_copy };
use crate::algebra::{ normalize_vector, dot_product };
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::iter::Iterator;
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::cmp::Ordering::Equal;

#[derive(juniper::GraphQLObject)]
#[derive(Queryable)]
pub(crate) struct Photo {
    pub id: i32,
    pub name: String,
    pub marker: String,
    pub file: String,
}

#[derive(Queryable)]
pub(crate) struct DestinationMarkerOccurrences {
    pub art_pois: i32,
    pub architecture_pois: i32,
    pub park_pois: i32,
    pub going_out_pois: i32,
    pub all_pois: i32,
    pub destination: String,
    pub id: i32,
}

#[derive(juniper::GraphQLObject)]
pub(crate) struct DestinationScore {
    pub destination: String,
    pub score: f64,
}

lazy_static! {
    static ref MARKERS_ORDER: HashMap<&'static str, usize> = {
        let mut map = HashMap::new();
        map.insert("art", 0);
        map.insert("architecture", 1);
        map.insert("park", 2);
        map.insert("going_out", 3);
        map
    };
}

#[derive(juniper::GraphQLInputObject)]
pub(crate) struct UserPreference {
    #[graphql]
    marker: String,

    #[graphql]
    like: bool,
}

const PHOTOS_CATEGORIES: [&str; 4] = [ "art", "architecture", "park", "going_out" ];

fn trim_photos_marker(photos_to_map: &mut Vec<Photo>) -> () {
    for i in 0..photos_to_map.len() {
        photos_to_map[i].marker = photos_to_map[i].marker.trim().to_string();
    }
}

impl crate::query::Query {
    pub(crate) fn photos_implementation(&self, context: &crate::context::Context, count_per_category: i32) -> FieldResult<Vec<Photo>> {
        let connection = context.pool.get().unwrap();

        let mut all_found_photos: Vec<Photo> = Vec::with_capacity(count_per_category as usize);

        PHOTOS_CATEGORIES.iter().for_each(|category| {
            let mut found_photos = photos
                .filter(marker.eq(category))
                .limit(count_per_category as i64)
                .load::<Photo>(&*connection).unwrap();
            trim_photos_marker(&mut found_photos);
            all_found_photos.append(&mut found_photos);
        });

        let mut rng = thread_rng();
        all_found_photos.shuffle(&mut rng);

        Ok(all_found_photos)
    }

    pub(crate) fn recommendation_implementation(&self, context: &crate::context::Context, user_preferences: Vec<UserPreference>) -> FieldResult<Vec<DestinationScore>> {
        let connection = context.pool.get().unwrap();

        let marker_stats = destination_marker_stats_copy
            .load::<DestinationMarkerOccurrences>(&*connection).unwrap();

        let mut destination_scores = Vec::with_capacity(marker_stats.len());

        let mut user_profile: [f64; 4] = [0.0f64; 4];
        for pref in user_preferences.iter() {
            let index: usize = *MARKERS_ORDER.get(pref.marker.as_str()).unwrap();
            if pref.like {
                user_profile[index] += 1.0f64;
            } else {
                user_profile[index] -= 1.0f64;
            }
        }
        normalize_vector(&mut user_profile);

        for marker_stats in marker_stats.iter() {
            let mut marker_vector = [
                marker_stats.art_pois as f64,
                marker_stats.architecture_pois as f64,
                marker_stats.park_pois as f64,
                marker_stats.going_out_pois as f64,
            ];
            normalize_vector(&mut marker_vector);

            let similarity = dot_product(&marker_vector, &user_profile);
            destination_scores.push(DestinationScore {
                destination: marker_stats.destination.to_owned(),
                score: similarity,
            });
        }

        destination_scores.sort_by(| d1, d2 | {
            d2.score.partial_cmp(&d1.score).unwrap_or(Equal)
        });

        Ok(destination_scores)
    }
}
