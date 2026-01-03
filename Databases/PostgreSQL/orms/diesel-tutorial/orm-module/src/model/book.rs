use crate::model::user::User;
use crate::schema::book_schema::books;
use crate::schema::book_schema::pages;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq, Associations)]
#[diesel(belongs_to(User, foreign_key = author_id))]
#[diesel(table_name = books)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author_id: Option<i32>,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(Book))]
#[diesel(table_name = pages)]
pub struct Page {
    pub id: i32,
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = books)]
pub struct NewBook {
    pub title: String,
    pub author_id: Option<i32>,
}

#[derive(Insertable)]
#[diesel(table_name = pages)]
pub struct NewPage {
    pub page_number: i32,
    pub content: String,
    pub book_id: i32,
}
