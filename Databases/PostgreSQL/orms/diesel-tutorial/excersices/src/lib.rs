pub fn build_database_url(
    host: &str,
    port: u16,
    database: &str,
    user: &str,
    password: &str,
) -> String {
    format!("postgres://{user}:{password}@{host}:{port}/{database}")
}