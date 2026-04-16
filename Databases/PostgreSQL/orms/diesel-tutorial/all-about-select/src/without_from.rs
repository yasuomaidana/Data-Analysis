use diesel::dsl::select;
use diesel::{RunQueryDsl, declare_sql_function};

#[declare_sql_function]
extern "SQL" {
    fn version() -> Text;
}
fn main() {
    let mut conn = orm_module::establish_connection();
    let version_text = select(version()).get_result::<String>(&mut conn);
    println!("Running Postgres v{:?}", version_text)
}
