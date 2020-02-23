table! {
    destination_marker_stats_copy (id) {
        art_pois -> Int4,
        architecture_pois -> Int4,
        park_pois -> Int4,
        going_out_pois -> Int4,
        all_pois -> Int4,
        destination -> Varchar,
        id -> Int4,
    }
}

table! {
    photos (id) {
        id -> Int4,
        name -> Varchar,
        marker -> Bpchar,
        file -> Varchar,
    }
}

table! {
    points_of_interest (id) {
        id -> Int4,
        data -> Jsonb,
    }
}

allow_tables_to_appear_in_same_query!(
    destination_marker_stats_copy,
    photos,
    points_of_interest,
);
