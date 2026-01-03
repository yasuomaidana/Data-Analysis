use clap::Parser;
use diesel::{
    ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper,
};
use orm_module::model::user::User;
use orm_module::schema::user_schema::users;

#[derive(Parser, Debug)]
#[command(author, version, about = "Create a book", long_about = None)]
struct Args {
    /// Optional user name
    #[arg(long = "user-name")]
    user_name: Option<String>,

    /// Optional user email
    #[arg(long = "user-email")]
    user_email: Option<String>,

    /// Book title (required)
    // #[arg(long = "book-title")]
    book_title: String,

    /// Pages content (zero or more)
    #[arg(value_name = "PAGES", num_args = 0..)]
    pages: Vec<String>,
}

fn main() {
    let args = Args::parse();
    let mut connection = orm_module::establish_connection();
    println!("Creating book: {}", args.book_title);
    let user: Option<User>;
    if let Some(name_input) = &args.user_name {
        let users = users::table
            .filter(users::name.eq(name_input))
            .select(User::as_select())
            .get_results(&mut connection)
            .expect("Failed to get users");
        if users.is_empty() {
            println!("No user found with name: {}", name_input);
            user = None;
        } else if users.len() > 1 {
            panic!(
                "Multiple users found with name: {} Try using email",
                name_input
            );
        } else {
            user = Some(users.into_iter().next().unwrap());
        }
    } else if let Some(email) = &args.user_email {
        user = users::table
            .filter(users::email.eq(email))
            .select(User::as_select())
            .get_result(&mut connection)
            .optional()
            .expect("Failed to get users");
    } else {
        user = None;
    }

    match &user {
        Some(u) => println!("Using user: {:#?}", u),
        None => println!("No user specified or found"),
    }

    println!("Pages ({}):", args.pages.len());
    for (i, page) in args.pages.iter().enumerate() {
        println!("  {}: {}", i + 1, page);
    }
}
