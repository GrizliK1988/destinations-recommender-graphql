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
    photos,
    points_of_interest,
);
