use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;
use orm_module::establish_connection;
use std::env;

// Embedding migrations!!!
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn main() {
    match env::var("ENV_CONFIG") {
        Ok(env_file) => {
            dotenvy::from_path(env_file).ok();
        }
        Err(_) => {
            dotenv().ok();
        }
    }
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", db_url);

    let mut connection = establish_connection();

    // This runs the required migrations.
    // match connection.run_pending_migrations(MIGRATIONS) {
    //     Ok(_) => println!("Migrations run successfully"),
    //     Err(e) => println!("Error running migrations: {}", e),
    // }

    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(applied) => {
            if applied.is_empty() {
                println!("No pending migrations to run");
            } else {
                println!("Applied migrations:");
                for m in applied {
                    println!("- {}", m);
                }
            }
        }
        Err(e) => eprintln!("Error running migrations: {}", e),
    }
}
