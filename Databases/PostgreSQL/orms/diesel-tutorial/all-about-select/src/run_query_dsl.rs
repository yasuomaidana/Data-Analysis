use diesel::internal::derives::multiconnection::chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use orm_module::model::user::UserRole;
use orm_module::schema::user_schema::users;

fn main() {
    let mut conn = orm_module::establish_connection();
    let all_users = users::table
        .load::<(
            i32,
            String,
            String,
            UserRole,
            Option<NaiveDate>,
            NaiveDateTime,
        )>(&mut conn)
        .expect("Error loading users");
    for user in all_users {
        println!("{:?}", user);
    }
}
