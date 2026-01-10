use clap::Parser;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, Insertable, QueryDsl, RunQueryDsl, insert_into,
};
use orm_module::establish_connection;
use orm_module::model::course::Enrollment;
use orm_module::schema::courses_schema::{courses, enrollments};
use orm_module::schema::user_schema::users;

#[derive(Parser, Debug)]
#[command(author, version, about = "Enroll an existing user to a course", long_about = None)]
struct Args {
    #[arg(help = "Username or email")]
    user_name: String,

    #[arg(help = "Course title")]
    course: String,
}

fn main() {
    let args = Args::parse();
    let mut connection = establish_connection();
    let user_id: i32 = users::table
        .filter(
            users::name
                .eq(&args.user_name)
                .or(users::email.eq(&args.user_name)),
        )
        .select(users::id)
        .get_result(&mut connection)
        .expect("Failed to find user");
    let course_id: i32 = insert_into(courses::table)
        .values((courses::title.eq(&args.course),))
        .on_conflict(courses::title)
        .do_nothing()
        .returning(courses::id)
        .get_result(&mut connection)
        .expect("Failed to upsert course");

    let enrollment = Enrollment {
        course_id,
        student_id: user_id,
    };

    enrollment
        .insert_into(enrollments::table)
        .on_conflict((enrollments::course_id, enrollments::student_id))
        .do_nothing()
        .execute(&mut connection)
        .expect("Failed to insert enrollment");
}
