use clap::Parser;
use diesel::associations::HasTable;
use diesel::{BelongingToDsl, QueryDsl, RunQueryDsl, SelectableHelper};
use orm_module::model::course::{Course, Enrollment};
use orm_module::model::user::User;
use orm_module::schema::courses_schema::courses;
use orm_module::schema::courses_schema::enrollments;
use orm_module::schema::user_schema::users;

#[derive(Parser, Debug)]
#[command(about = "Show enrollments")]
struct Args {
    #[arg(short = 'm', long = "max")]
    max: Option<i32>,

    #[arg(help = "Username or email", short = 'u', long = "user")]
    user_name: Option<String>,

    #[arg(help = "Course title", short = 'c', long = "course")]
    course: Option<String>,
}

fn get_all(connection: &mut diesel::PgConnection) {
    let enrolled_student_ids = enrollments::table
        .select(enrollments::student_id)
        .distinct()
        .get_results::<i32>(connection)
        .expect("Error loading enrollments");

    if enrolled_student_ids.is_empty() {
        println!("No enrollments found");
    } else {
        for enrollment in enrolled_student_ids {
            let student = users::table
                .find(enrollment)
                .select(User::as_select())
                .get_result::<User>(connection)
                .expect("Error loading student");

            let student_enrollments = Enrollment::belonging_to(&student)
                .inner_join(courses::table)
                .select(Course::as_select())
                .load::<Course>(connection)
                .expect("Error loading enrollments for student");

        }
    }
}
fn main() {
    let args = Args::parse();
}
