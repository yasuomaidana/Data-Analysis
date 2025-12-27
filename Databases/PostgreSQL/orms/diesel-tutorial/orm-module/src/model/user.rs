use crate::schema::user_schema::users;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::{AsChangeset, Identifiable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;

#[derive(DbEnum, Debug)]
#[db_enum(existing_type_path = "crate::schema::user_schema::sql_types::UserRole")]
pub enum UserRole {
    Guest,
    User,
    Moderator,
    Admin,
}

#[derive(Queryable, Selectable, Debug, Identifiable, AsChangeset)]
#[diesel(table_name = users, check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    name: String,
    email: String,
    role: UserRole,
    birth_date: Option<NaiveDate>,
    // It includes date and time
    updated: NaiveDateTime,
}
