use crate::model::user::User;
use crate::schema::courses_schema::{courses, enrollments};

use diesel::{Associations, Identifiable, Queryable, Selectable};

#[derive(Identifiable, Selectable, Queryable, Debug)]
pub struct Course {
    pub id: i32,
    pub title: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug)]
#[diesel(belongs_to(Course))]
#[diesel(belongs_to(User, foreign_key = student_id))]
#[diesel(table_name = enrollments)]
#[diesel(primary_key(course_id, student_id))]
pub struct Enrollment {
    pub course_id: i32,
    pub student_id: i32,
}
