use std::env;

fn main() {
    match env::var("DATABASE_URL") {
        Ok(v) => println!("DATABASE_URL is: {}", v),
        Err(e) => eprintln!("DATABASE_URL not set: {}", e),
    }

    match env::var("MY_WHOLE") {
        Ok(v) => println!("MY_WHOLE is: {}", v),
        Err(e) => eprintln!("MY_WHOLE not set: {}", e),
    }
}
