// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Int4,
        name -> Varchar,
        publoc -> Bool
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    card,
    posts,
);
