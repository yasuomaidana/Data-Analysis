use clap::Parser;
use diesel::prelude::*;
use diesel::sql_types::{Integer, Record, Text};
use itertools::Itertools;

#[derive(Parser, Debug)]
#[command(author, version, about = "Create a book", long_about = None)]
struct Args {
    #[arg(value_name = "COURSE", help = "Course name to search")]
    course: String,
}

define_sql_function! {
    #[sql_name = "get_users_enrolled_in_course"]
    fn get_users_enrolled_in_course(course_name: Text) -> Record<(Integer, Text, Text)>;
}

#[derive(QueryableByName, Debug)]
struct UserRecord {
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Text)]
    name: String,
    #[diesel(sql_type = Text)]
    email: String,
}

fn main() {
    let mut conn = orm_module::establish_connection();
    let args = Args::parse();
    let course = args.course;
    println!("Searching for users enrolled in {}", &course);

    let results = diesel::select(get_users_enrolled_in_course(course))
        .load::<(i32, String, String)>(&mut conn)
        .unwrap_or_else(|e| {
            eprintln!("Query failed: {}", e);
            Vec::new()
        });

    let result = results
        .iter()
        .map(|(id, name, email)| format!("\t{} - {} - {}", id, name, email))
        .join("\n");
    println!("{result}");
}
