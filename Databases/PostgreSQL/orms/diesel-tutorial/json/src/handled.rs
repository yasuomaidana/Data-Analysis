use diesel::{BoolExpressionMethods, ExpressionMethods, TextExpressionMethods};
use diesel::{QueryDsl, RunQueryDsl};
use diesel_json::Json;
use json::{JTrack, Track};
use orm_module::establish_connection;
use orm_module::schema::jtrack_schema::jtrack;

fn main() {
    let mut conn = establish_connection();

    let query = jtrack::table.filter(
        Track::count_sql()
            .ge(20)
            .and(Track::name_sql().like("%Summer%")),
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
