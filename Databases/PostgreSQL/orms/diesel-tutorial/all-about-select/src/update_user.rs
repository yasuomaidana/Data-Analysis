use diesel::dsl::update;
use diesel::internal::derives::multiconnection::chrono::NaiveDate;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use orm_module::establish_connection;
use orm_module::model::user::{UpdateUser, User};
use orm_module::schema::user_schema::users;
use utils::reader::{get_arg, get_arg_or_prompt};
mod utils_;
use crate::utils_::random_birth_date;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut connection = establish_connection();

    let user_id: i32 = get_arg_or_prompt(&args, 1, "Enter user id: ");
    let name: Option<String> = match get_arg::<String>(&args, 2) {
        Ok(Some(s)) if s.trim().len() > 1 => Some(s),
        _ => None,
    };
    let email: Option<String> = match get_arg::<String>(&args, 3) {
        Ok(Some(s)) if s.trim().len() > 1 => Some(s),
        _ => None,
    };
    let birth_date: Option<Option<NaiveDate>> = match get_arg::<String>(&args, 4) {
        Ok(Some(s)) if s.to_uppercase() == "Y" => Some(Some(random_birth_date())),
        Ok(Some(s)) if s.to_uppercase() == "N" => Some(None),
        _ => None,
    };

    let rows_updated: Vec<User> = update(users::table.find(user_id))
        .set(UpdateUser {
            name,
            email,
            birth_date,
        })
        .returning(User::as_returning())
        .get_results(&mut connection)
        .expect("Failed to update user");

    if rows_updated.is_empty() {
        println!("No users where updated, available users are:");
        let res = users::table
            .select((users::id, users::name))
            .get_results::<(i32, String)>(&mut connection)
            .expect("Error loading users");
        for user in res {
            println!("{:?}", user);
        }
    } else {
        let user = &rows_updated[0];
        println!("Updated user: {:?}", user);
        // println!("Updated user: {:?}", rows_updated[0]);
    }
}
