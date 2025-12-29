use crate::schema::user_schema::users;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::dsl::insert_into;
use diesel::result::Error;
use diesel::{
    AsChangeset, Identifiable, Insertable, OptionalExtension, Queryable, RunQueryDsl, Selectable,
};
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

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub role: Option<UserRole>,
    pub birth_date: Option<NaiveDate>,
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
