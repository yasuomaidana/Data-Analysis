use crate::schema::user_schema::users;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::dsl::insert_into;
use diesel::result::Error;
use diesel::{
    AsChangeset, HasQuery, Identifiable, Insertable, OptionalExtension, Queryable, RunQueryDsl,
    Selectable,
};
use diesel_derive_enum::DbEnum;
use serde::Deserialize;
use std::fmt::Debug;
use std::str::FromStr;
use utils::reader::ReaderError;

#[derive(DbEnum, Debug, Deserialize)]
#[db_enum(existing_type_path = "crate::schema::user_schema::sql_types::UserRole")]
pub enum UserRole {
    Guest,
    User,
    Moderator,
    Admin,
}

//Identifiable with AsChangeset allows to use save method
#[derive(Queryable, Selectable, Debug, Identifiable, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub birth_date: Option<NaiveDate>,
    // It includes date and time
    pub updated: NaiveDateTime,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub email: Option<String>,
    pub birth_date: Option<Option<NaiveDate>>,
}

#[derive(Insertable, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub role: Option<UserRole>,
    pub birth_date: Option<NaiveDate>,
}

impl FromStr for NewUser {
    type Err = ReaderError<serde_json::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let user = serde_json::from_str(s).map_err(|e| ReaderError::ParseError(e.into()))?;
        Ok(user)
    }
}

#[derive(HasQuery, Debug)]
#[diesel(table_name = users)]
pub struct UserEmail {
    pub id: i32,
    pub name: String,
    pub email: String,
}

impl NewUser {
    pub fn quick_new(name: String, email: String) -> Self {
        Self {
            name,
            email,
            role: None,
            birth_date: None,
        }
    }
    pub fn create(&self, conn: &mut diesel::PgConnection) -> Result<Option<User>, Error> {
        insert_into(users::table)
            .values(self)
            .get_result(conn)
            .optional()
    }
}
