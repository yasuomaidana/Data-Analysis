use crate::postgres_config::{PostgresConfig, read_conf_from_env};
use postgres::{Client, Config, GenericClient, NoTls};
use reqwest::blocking;
use std::fs::File;
use std::io::BufReader;
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

pub fn copy_csv_to_postgres(config: PostgresConfig, csv_file_path: &str) {
    let connection_string = config.get_connection_string();
    println!("{:?}", config);
    println!("{connection_string}");
    let mut client =
        Client::connect(&connection_string, NoTls).expect("Failed to connect to the database");
    
    // Start the COPY stream
    let mut copy_in = client
        .copy_in("COPY track_raw(title, artist, album, count, rating, len) FROM STDIN WITH CSV")
        .expect("Failed to start COPY");

    // Open and read the CSV file
    let file = File::open(csv_file_path).expect("Failed to open CSV");
    let mut reader = BufReader::new(file);

    // Stream contents to the COPY process
    std::io::copy(&mut reader, &mut copy_in).expect("Failed to copy data to database");
    copy_in.finish().expect("Failed to finish copy");

    println!("Data successfully imported from '{}'.", csv_file_path);
}
