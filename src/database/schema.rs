// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        a_type -> Varchar,
        answer -> Varchar,
    }
}
