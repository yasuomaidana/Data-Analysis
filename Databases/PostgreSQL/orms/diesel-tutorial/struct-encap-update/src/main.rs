use clap::Parser;
use diesel::dsl::select;
use diesel::{BelongingToDsl, ExpressionMethods, QueryDsl, RunQueryDsl};
use orm_module::establish_connection;
use orm_module::model::user::{User, UserConfig, UserConfigBuilder};
use orm_module::schema::user_schema::users;

#[derive(Parser, Debug)]
#[command(author, version, about = "Update user's settings", long_about = None)]
struct Args {
    #[arg(help = "User name or email")]
    user: String,

    #[arg(help = "New user's points")]
    points: i32,

    #[arg(long, short, help = "Enabled", default_value_t = true)]
    enabled: bool,
}

fn main() {
    println!("Updating car using struct encapsulation");
    let args = Args::parse();
    let mut connection = establish_connection();

    let user = if args.user.contains('@') {
        users::table
            .filter(users::email.eq(args.user))
            .first::<User>(&mut connection)
    } else {
        users::table
            .filter(users::name.eq(args.user))
            .first::<User>(&mut connection)
    };

    let user = match user {
        Ok(u) => u,
        Err(_) => {
            eprintln!("User not found");
            std::process::exit(1);
        }
    };

    let config = UserConfig::belonging_to(&user).first::<UserConfig>(&mut connection);

    match config {
        Ok(c) => {
            c;
        }
        Err(_) => {
            println!("No config was found for user");
            let config_builder = UserConfigBuilder::default().points(args.points).enabled(args.enabled).build();
            
            
            
        }
    }

    println!("{:?}", user);
    // TODO: Implementation deferred — requires many-to-many (m-to-n) relationship support in ORM/schema.
    // Once m-to-n associations are defined, implement the update using struct encapsulation and handle results.

    // let args: Vec<String> = std::env::args().collect();
    //
    // let model_input: String = get_arg_or_prompt(&args, 1, "Enter model: ");
    // let year_input: i32 = get_arg_or_default(&args, 2, 2015);
    //
    // let mut connection = establish_connection();
    //
    // let stored_car = cars
    //     .filter(year.eq(year_input).and(model.eq(model_input)))
    //     .get_result::<Car>(&mut connection);
    //
    // match stored_car {
    //     Ok(car) => {
    //         println!("Found car: {:?}", car);
    //         update(Car::belonging_to(car))
    //     }
    //     Err(e) => {}
    // }
}
