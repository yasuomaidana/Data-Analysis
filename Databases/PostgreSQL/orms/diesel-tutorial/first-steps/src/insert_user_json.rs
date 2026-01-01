use diesel::dsl::insert_into;
use diesel::result::DatabaseErrorKind;
use diesel::result::Error::DatabaseError;

use orm_module::model::user::{NewUser, User};

use diesel::upsert::excluded;
use diesel::{ExpressionMethods, RunQueryDsl, SelectableHelper};
use orm_module::schema::user_schema::users::dsl::users;
use orm_module::schema::user_schema::users::{birth_date, email, name, role};
use utils::reader::get_arg_or_prompt;

fn main() {
    println!("Insert a user passing a json with this format:");
    println!(
        "{}",
        r#"
    Example JSON to pass as a single command-line argument (no extra wrapping quotes):
    
    {
      "name": "Alice Example",
      "email": "alice@example.com",
      "role": "Admin",
      "birth_date": "1990-05-20"
    }

    "#
    );
    println!("Or with optional fields omitted/null:");
    println!(
        "{}",
        r#"
    {
      "name": "Bob Example",
      "email": "bob@example.com",
      "role": null,
      "birth_date": null
    }
    "#
    );
    let args = std::env::args().collect::<Vec<String>>();
    let new_user: NewUser = get_arg_or_prompt(&args, 1, "Pass a user json: ");
    println!("Inserting user: {:#?}", new_user);
    let stored_new = insert_into(users)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(&mut orm_module::establish_connection());
    match stored_new {
        Ok(user) => println!("Inserted user: {:#?}", user),
        Err(DatabaseError(DatabaseErrorKind::UniqueViolation, info)) => {
            println!("{}, applying upsert", info.message());
            let conn = &mut orm_module::establish_connection();
            let upserted = insert_into(users)
                .values(&new_user)
                .on_conflict(email)
                .do_update()
                .set((
                    name.eq(excluded(name)),
                    role.eq(excluded(role)),
                    birth_date.eq(excluded(birth_date)),
                ))
                .returning(User::as_returning())
                .get_result(conn);
            match upserted {
                Ok(u) => println!("Upserted user: {:#?}", u),
                Err(e) => println!("Error upserting user: {:#?}", e),
            }
        }
        Err(e) => println!("Error inserting user: {:#?}", e),
    }
}
