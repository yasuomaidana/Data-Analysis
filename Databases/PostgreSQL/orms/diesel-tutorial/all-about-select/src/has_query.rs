use diesel::pg::Pg;
use diesel::{ExpressionMethods, HasQuery, PgConnection};
use diesel::{QueryDsl, RunQueryDsl};
use orm_module::establish_connection;
use orm_module::model::user::{UserEmail, UserRole};
use orm_module::schema::user_schema::users;

fn execute_select<Q, U>(query: Q, connection: &mut PgConnection, message: Option<&str>)
where
    // Require that `Q` can be loaded from a `PgConnection` for any lifetime `'a`,
    // producing values of type `U`. This bound enables calling `query.load(connection)`
    // to fetch rows and deserialize them into `U`.
    for<'a> Q: diesel::query_dsl::LoadQuery<'a, PgConnection, U>,
    Q: diesel::query_builder::QueryFragment<Pg>
        + diesel::query_builder::QueryId
        + RunQueryDsl<PgConnection>,
    U: std::fmt::Debug,
{
    println!("SQL: {}", diesel::debug_query::<Pg, _>(&query));

    if let Some(message) = message {
        println!("{}", message);
    }

    for user in query.load(connection).expect("Error loading users") {
        println!("{:?}", user);
    }
}

fn main() {
    let mut conn = establish_connection();

    let query = UserEmail::query()
        .filter(users::role.eq(UserRole::Admin).eq(false))
        .order_by(users::id);

    execute_select(query, &mut conn, Some("Non admin users"));

    // simple version
    let query = UserEmail::query()
        .filter(users::role.ne(UserRole::Admin))
        .order_by(users::id);

    execute_select(query, &mut conn, Some("Simple version, non admin users"));
}
