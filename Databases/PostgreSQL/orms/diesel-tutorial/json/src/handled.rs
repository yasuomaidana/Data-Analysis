use diesel::dsl::sql;
use diesel::sql_types::{Integer, Text};
use diesel::{
    BoolExpressionMethods, ExpressionMethods, PgTextExpressionMethods, TextExpressionMethods,
};
use diesel::{QueryDsl, RunQueryDsl};
use diesel_json::Json;
use json::JTrack;
use orm_module::establish_connection;
use orm_module::schema::jtrack_schema::jtrack;

fn main() {
    let mut conn = establish_connection();

    let query = jtrack::table.filter(
        sql::<Integer>("(body->>'count')::int")
            .ge(20)
            .and(sql::<Text>("(body->>'name')::text").like("%Summer%")),
    );
    println!("SQL: {}", diesel::debug_query::<diesel::pg::Pg, _>(&query));

    let results = query.load::<JTrack>(&mut conn).map_or(vec![], |res| res);

    println!("Results: {:?}", &results.len());

    results.iter().for_each(|res| match &res.body {
        Some(Json(body)) => {
            println!("name: {}, count: {}", body.name, body.count);
        }
        None => {
            println!("name: <none>, count: <none>");
        }
    });
}
