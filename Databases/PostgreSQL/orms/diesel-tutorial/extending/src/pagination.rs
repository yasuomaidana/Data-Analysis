use diesel::associations::HasTable;
use diesel::query_builder::AsQuery;
use diesel_pagination::{Paginate, PaginationParams};
use orm_module::model::graph::Relationship;

fn main() {
    let mut conn = orm_module::establish_connection();

    let result = Relationship::table()
        .as_query()
        .paginate(PaginationParams {
            page: Some(1),
            per_page: Some(10),
        })
        .load_and_count::<Relationship, _>(&mut conn)
        .unwrap();
    println!("{:?}", result);
}
