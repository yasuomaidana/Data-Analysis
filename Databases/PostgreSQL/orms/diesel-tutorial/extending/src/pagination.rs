use clap::Parser;
use diesel::associations::HasTable;
use diesel::query_builder::AsQuery;
use diesel_pagination::{Paginate, PaginationParams};
use orm_module::model::graph::Relationship;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 1)]
    page: i64,
    #[arg(short = 'o', long, default_value_t = 1)]
    per_page: i64,
}

fn main() {
    let args = Cli::parse();
    let mut conn = orm_module::establish_connection();

    let result = Relationship::table()
        .as_query()
        .paginate(PaginationParams {
            page: Some(args.page),
            per_page: Some(args.per_page),
        })
        .load_and_count::<Relationship, _>(&mut conn)
        .unwrap();
    println!("page: {}", result.page);
    println!("per_page: {}", result.per_page);
    println!("total: {}", result.num_total);
    for rel in result.items {
        println!("{:?}", rel);
    }
}
