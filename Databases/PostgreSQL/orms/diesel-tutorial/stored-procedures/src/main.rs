use clap::Parser;
use diesel::sql_types::Text;
use diesel::{RunQueryDsl, sql_query};
use itertools::Itertools;
use orm_module::schema::user_schema::users;

#[derive(Parser, Debug)]
#[command(author, version, about = "Create a book", long_about = None)]
struct Args {
    #[arg(value_name = "COURSE", help = "Course name to search")]
    course: String,
}

#[derive(diesel::QueryableByName, Debug)]
#[diesel(table_name = users)]
struct UserEmail {
    pub name: String,
    pub email: String,
}

fn main() {
    let mut conn = orm_module::establish_connection();
    let args = Args::parse();
    let course = args.course;
    println!("Searching for users enrolled in {}", &course);

    let results: Vec<UserEmail> = sql_query(
        "SELECT u.name, u.email FROM get_users_enrolled_in_course($1) AS u(_,name, email)",
    )
    .bind::<Text, _>(course)
    .get_results(&mut conn)
    .unwrap_or_else(|e| {
        eprintln!("Query failed: {}", e);
        Vec::new()
    });

    let result = results
        .iter()
        .map(|user| format!("\t{} - {}", user.name, user.email))
        .join("\n");
    println!("{result}");
}
