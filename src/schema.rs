// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        title -> Text,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Nullable<Timestamptz>,
    }
}
