// @generated automatically by Diesel CLI.

diesel::table! {
    students (id) {
        id -> Text,
        name_student -> Text,
        created_at -> Timestamp,
    }
}
