mod utils_;

use diesel::{insert_into, RunQueryDsl};
use orm_module::model::user::{NewUser, UserRole};
use orm_module::schema::user_schema::users::dsl::users;
use rand::distr::Alphanumeric;
use rand::{random_bool, random_range, rng, RngExt};
use crate::utils_::random_birth_date;

fn random_string(len: usize) -> String {
    rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

fn random_length_string(min: usize, max: usize) -> String {
    let len = rng().random_range(min..=max);
    random_string(len)
}

fn generate_quick_user() -> NewUser {
    let name = random_length_string(5, 15);
    let email = format!("{}@example.com", random_length_string(5, 10));
    NewUser::quick_new(name, email)
}

fn generate_random_user() -> NewUser {
    let name = random_length_string(5, 15);
    let email = format!("{}@example.com", random_length_string(5, 10));
    let role = Some(match random_range(0..3) {
        0 => UserRole::Admin,
        1 => UserRole::Guest,
        2 => UserRole::Moderator,
        _ => UserRole::User,
    });

    let birth_date = if random_bool(2.0 / 3.0) {
        Some(random_birth_date())
    } else {
        None
    };

    NewUser {
        name,
        email,
        role,
        birth_date,
    }
}

fn main() {
    let quick_users = (0..5)
        .map(|_| generate_quick_user())
        .chain((0..20).map(|_| generate_random_user()))
        .collect::<Vec<NewUser>>();

    let mut conn = orm_module::establish_connection();
    insert_into(users)
        .values(&quick_users)
        .execute(&mut conn)
        .expect("Failed to save users");

    let stored_users = users
        .load::<orm_module::model::user::User>(&mut conn)
        .expect("Failed to load users");
    for user in stored_users {
        println!("{:?}", user);
    }
}
