use diesel::internal::derives::multiconnection::chrono::NaiveDateTime;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use orm_module::establish_connection;
use orm_module::model::user::UserRole;
use orm_module::schema::user_schema::users;

fn main() {
    let mut conn = establish_connection();
    let exp = users::table
        .select((users::name, users::role, users::updated, users::email))
        .filter(users::role.eq(UserRole::Admin))
        .order_by((users::birth_date.desc(), users::updated.desc()));

    let sql = diesel::debug_query::<diesel::pg::Pg, _>(&exp).to_string();
    println!("Generated SQL: {}", sql);

    let admin_users = exp
        .load::<(String, UserRole, NaiveDateTime, String)>(&mut conn)
        .expect("Error loading users");

    for user in admin_users {
        println!("{:?}", user);
    }
}
