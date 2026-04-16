pub fn build_database_url(
    host: &str,
    port: u16,
    database: &str,
    user: &str,
    password: &str,
) -> String {
    format!("postgres://{user}:{password}@{host}:{port}/{database}")
}

#[macro_export]
macro_rules! impl_get_connection {
    ($ty:ident) => {
        impl $ty {
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
    };
}