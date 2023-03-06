// @generated automatically by Diesel CLI.

diesel::table! {
    answers (id) {
        id -> Int4,
        question_id -> Int4,
        answer -> Varchar,
    }
}

diesel::table! {
    paper_answers (id) {
        id -> Int4,
        paper_id -> Int4,
        answer_id -> Int4,
    }
}

diesel::table! {
    papers (id) {
        id -> Int4,
        form_id -> Int4,
        user_id -> Int4,
    }
}

diesel::joinable!(paper_answers -> answers (answer_id));
diesel::joinable!(paper_answers -> papers (paper_id));

diesel::allow_tables_to_appear_in_same_query!(
    answers,
    paper_answers,
    papers,
);
