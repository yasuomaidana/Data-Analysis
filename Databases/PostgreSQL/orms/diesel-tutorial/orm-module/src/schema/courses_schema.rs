// @generated automatically by Diesel CLI.

use crate::schema::user_schema::users;

diesel::table! {
    courses (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
    }
}

diesel::table! {
    enrollments (student_id, course_id) {
        student_id -> Int4,
        course_id -> Int4,
    }
}

diesel::joinable!(enrollments -> users (student_id));
diesel::joinable!(enrollments -> courses (course_id));

diesel::allow_tables_to_appear_in_same_query!(courses, enrollments, users);
