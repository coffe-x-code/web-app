// @generated automatically by Diesel CLI.

diesel::table! {
    product (id) {
        id -> Int4,
        title -> Varchar,
        description -> Varchar,
        image_url -> Varchar,
        published -> Bool,
    }
}
