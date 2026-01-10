use crate::model::user::User;
use crate::schema::courses_schema::{courses, enrollments};
use crate::schema::user_schema::users;

use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

#[derive(Identifiable, Selectable, Queryable, Debug)]
pub struct Course {
    pub id: i32,
    pub title: String,
}

#[derive(Identifiable, Selectable, Queryable, Associations, Debug, Insertable)]
#[diesel(belongs_to(Course, foreign_key = course_id))]
#[diesel(belongs_to(User, foreign_key = student_id))]
#[diesel(table_name = enrollments)]
#[diesel(primary_key(student_id, course_id))]
pub struct Enrollment {
    pub student_id: i32,
    pub course_id: i32,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = users)]
pub struct Student {
    pub name: String,
    pub email: String,
}
pub struct StudentData {
    pub student: User,
    pub courses: Vec<Course>,
}
