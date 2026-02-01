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

#[derive(QueryableByName, Debug, Queryable)]
struct UserRecord {
    #[diesel(sql_type = Integer)]
    user_id: i32,
    #[diesel(sql_type = Text)]
    user_name: String,
    #[diesel(sql_type = Text)]
    user_email: String,
}

// Implement Queryable for the Record type returned by the function
impl diesel::deserialize::Queryable<Record<(Integer, Text, Text)>, diesel::pg::Pg> for UserRecord {
    type Row = (i32, String, String);

    fn build(row: Self::Row) -> diesel::deserialize::Result<Self> {
        Ok(UserRecord {
            user_id: row.0,
            user_name: row.1,
            user_email: row.2,
        })
    }
}

fn main() {
    let mut conn = orm_module::establish_connection();
    let args = Args::parse();
    let course = args.course;
    println!("Searching for users enrolled in {}", &course);

    let results = diesel::select(get_users_enrolled_in_course(course))
        .load::<UserRecord>(&mut conn)
        .unwrap_or_else(|e| {
            eprintln!("Query failed: {}", e);
            Vec::new()
        });

    let result = results
        .iter()
        .map(|user| {
            format!("\t{} - {} - {}", user.user_id, user.user_name, user.user_email)
        })
        .join("\n");
    println!("{result}");
}
