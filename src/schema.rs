// @generated automatically by Diesel CLI.

diesel::table! {
    links (id) {
        id -> Int4,
        original_link -> Text,
        short_link -> Text,
        created_at -> Timestamp,
    }
}
