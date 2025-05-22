use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

#[derive(Debug)]
pub struct PostgresConfig {
    host: String,
    port: u16,
    user: String,
    password: String,
    database: String,
}

impl PostgresConfig {
    pub fn get_connection_string(&self) -> String {
        format!(
            "host={} port={} user={} password={} dbname={}",
            self.host, self.port, self.user, self.password, self.database
        )
    }
}

pub fn read_conf_from_env(configuration_file: &PathBuf) -> PostgresConfig {
    let file = File::open(configuration_file).expect("Failed to open configuration file");
    let reader = BufReader::new(file);
    let mut config_map = std::collections::HashMap::new();
    config_map.insert("host".to_string(), "localhost".to_string());
    config_map.insert("port".to_string(), "5432".to_string());
    config_map.insert("user".to_string(), "user".to_string());
    config_map.insert("password".to_string(), "password".to_string());
    config_map.insert("database".to_string(), "database".to_string());

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(|c| c == '=' || c == ':').collect();
        if parts.len() != 2 {
            continue;
        }
        if config_map.contains_key(parts[0]) {
            config_map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    PostgresConfig {
        host: config_map.get("host").unwrap().to_string(),
        port: config_map
            .get("port")
            .unwrap()
            .parse::<u16>()
            .expect("Failed to parse port"),
        user: config_map.get("user").unwrap().to_string(),
        password: config_map.get("password").unwrap().to_string(),
        database: config_map.get("database").unwrap().to_string(),
    }
}
