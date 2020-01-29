use juniper::{FieldResult};
use diesel::Queryable;
use crate::diesel::RunQueryDsl;
use crate::diesel::prelude::*;
use crate::schema::photos::dsl::{ photos, marker };
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::iter::Iterator;

#[derive(juniper::GraphQLObject)]
#[derive(Queryable)]
pub(crate) struct Photo {
    pub id: i32,
    pub name: String,
    pub marker: String,
    pub file: String,
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
}
