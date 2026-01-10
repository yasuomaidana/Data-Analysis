use clap::Parser;
use diesel::{BelongingToDsl, ExpressionMethods, GroupedBy};
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use itertools::Itertools;
use orm_module::model::course::{Course, Enrollment, Student, StudentData};
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

    #[arg(
        help = "Course title",
        short = 'l',
        long = "long",
        default_value_t = false
    )]
    long: bool,
}

fn get_all(connection: &mut diesel::PgConnection, args: &Args) {
    let enrolled_student_ids = enrollments::table
        .select(enrollments::student_id)
        .distinct()
        .get_results::<i32>(connection)
        .expect("Error loading enrollments");

    if enrolled_student_ids.is_empty() {
        println!("No enrollments found");
    } else {
        match args.long {
            true => {
                for enrollment in enrolled_student_ids {
                    let user_courses = users::table
                        .find(enrollment)
                        .inner_join(enrollments::table.inner_join(courses::table))
                        .select((Student::as_select(), Course::as_select()))
                        .load::<(Student, Course)>(connection)
                        .expect("Error loading student and enrollments");

                    for (student, course) in user_courses {
                        println!("{} is enrolled in {}", student.name, course.title);
                    }
                    println!("---");
                }
            }
            false => {
                let users = users::table
                    .select(User::as_select())
                    .filter(users::id.eq_any(&enrolled_student_ids))
                    .load::<User>(connection)
                    .expect("Error loading users");

                let enrollment_course_pairs = Enrollment::belonging_to(&users)
                    .inner_join(courses::table)
                    .load::<(Enrollment, Course)>(connection)
                    .expect("Error loading enrollments and courses");

                let student_data = enrollment_course_pairs
                    .grouped_by(&users)
                    .into_iter()
                    .zip(users)
                    .map(|(enrollments_with_courses, user)| {
                        let courses = enrollments_with_courses
                            .into_iter()
                            .map(|(_enrollment, course)| course)
                            .collect::<Vec<Course>>();
                        StudentData {
                            student: user,
                            courses,
                        }
                    })
                    .collect::<Vec<StudentData>>();
                for student_data in student_data {
                    println!(
                        "{} is enrolled in: {}",
                        student_data.student.name,
                        student_data
                            .courses
                            .iter()
                            .map(|course| course.title.as_str())
                            .join(", ")
                    )
                }
            }
        }
    }
}
fn main() {
    let args = Args::parse();
    let mut connection = orm_module::establish_connection();
    get_all(&mut connection, &args);
}
