use crate::postgres_config::read_conf_from_env;
use postgres::{Client, NoTls};
use reqwest::blocking;
use std::fs::File;
use std::path::{Path, PathBuf};

pub fn download(url: &String) {
    if Path::new("library.csv").exists() {
        println!("File already exists, skipping download.");
        return;
    } else {
        println!("Downloading...");
        let response = blocking::get(url).expect("Could not fetch URL");
        if !response.status().is_success() {
            panic!("Failed to download file: {}", response.status());
        }
        let mut file = File::create("library.csv").expect("Failed to create file");
        std::io::copy(&mut response.bytes().unwrap().as_ref(), &mut file)
            .expect("Failed to write to file");
        println!("Download complete.");
    }
}

pub fn create_table(config: &PathBuf) {
    println!("Creating table...");
    let database_config = read_conf_from_env(config);
    let connection_string = database_config.get_connection_string();

    let mut client =
        Client::connect(&connection_string, NoTls).expect("Failed to connect to Postgres");
    let create_table_query = r#"
        CREATE TABLE IF NOT EXISTS track_raw
        (title TEXT, artist TEXT, album TEXT,
        count INTEGER, rating INTEGER, len INTEGER);
        "#;

    client
        .execute(create_table_query, &[])
        .expect("Failed to create table.");
    println!("Database config: {:?}", database_config);
    println!("Table created.");
}
