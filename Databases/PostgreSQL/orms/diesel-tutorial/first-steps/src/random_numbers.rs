use clap::Parser;
use diesel::dsl::insert_into;
use diesel::{Connection, ConnectionResult, Insertable, PgConnection, RunQueryDsl, sql_query};

pub fn build_database_url(
    host: &str,
    port: u16,
    database: &str,
    user: &str,
    password: &str,
) -> String {
    format!("postgres://{user}:{password}@{host}:{port}/{database}")
}

#[derive(Parser, Debug)]
#[command(author, version, about = "Create a book", long_about = None)]
struct Args {
    host: String,
    port: u16,
    database: String,
    user: String,
    password: String,
}

impl Args {
    fn get_connection(&self) -> ConnectionResult<PgConnection> {
        PgConnection::establish(&build_database_url(
            &self.host,
            self.port,
            &self.database,
            &self.user,
            &self.password,
        ))
    }
}

diesel::table! {
    pythonseq (iter) {
        iter -> Integer,
        val -> Integer,
    }
}

#[derive(Insertable)]
#[diesel(table_name = pythonseq)]
struct NewPythonSeq {
    iter: i32,
    val: i32,
}
fn main() {
    use crate::pythonseq::dsl::pythonseq;

    let args = Args::parse();
    let mut connection = args
        .get_connection()
        .expect("Failed to connect to the database");
    sql_query("CREATE TABLE IF NOT EXISTS pythonseq (iter INTEGER, val INTEGER);")
        .execute(&mut connection)
        .expect("Failed to create table");
    let mut value: u64 = 573_696;
    for i in 0..300 {
        println!("{} {}", i + 1, value);
        let new_row = NewPythonSeq {
            iter: i+1,
            val: value as i32,
        };
        insert_into(pythonseq)
            .values(&new_row)
            .execute(&mut connection)
            .expect("Failed to insert value");
        value = ((value * 22) / 7) % 1_000_000;
    }
}
