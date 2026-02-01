use clap::Parser;
use diesel::prelude::*;
//use diesel::sql_types::{Integer, Text};
use diesel::sql_types::Text;
use itertools::Itertools;
use std::fmt;
use std::fmt::Formatter;

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
    //  #[diesel(sql_type = Integer)]
    // _user_id: i32,
    #[diesel(sql_type = Text)]
    user_name: String,
    #[diesel(sql_type = Text)]
    user_email: String,
}

impl From<(i32, String, String)> for UserRecord {
    fn from(row: (i32, String, String)) -> Self {
        let (_, user_name, user_email) = row;
        UserRecord {
            user_name,
            user_email,
        }
    }
}

impl fmt::Display for UserRecord {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.user_name, self.user_email)
    }
}

fn main() {
    let mut conn = orm_module::establish_connection();
    let args = Args::parse();
    let course = args.course;
    println!("Searching for users enrolled in {}", &course);

    let results = diesel::select(get_users_enrolled_in_course(course))
        .load::<(i32, String, String)>(&mut conn)
        .map(|rows| {
            rows.into_iter()
                .map(UserRecord::from)
                .collect::<Vec<UserRecord>>()
        })
        .unwrap_or_else(|e| {
            eprintln!("Query failed: {}", e);
            Vec::new()
        });

    let result = results.iter().map(|user| format!("\t{:}", user)).join("\n");
    println!("{result}");
}
