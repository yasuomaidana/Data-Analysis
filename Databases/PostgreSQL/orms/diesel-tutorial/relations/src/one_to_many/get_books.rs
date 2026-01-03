use clap::Parser;
use diesel::RunQueryDsl;
use diesel::{BelongingToDsl, ExpressionMethods};
use diesel::{GroupedBy, QueryDsl};
use orm_module::model::book::{Book, BookWithPages, Page};
use orm_module::schema::book_schema::books;

#[derive(Parser, Debug)]
#[command(author, version, about = "Create a book", long_about = None)]
struct Args {
    #[arg(value_name = "NAME", help = "Book name to search")]
    book_name: Option<String>,
}

fn print_book_and_pages(book: &Book, pages: &Vec<Page>) {
    println!("Book: {:?}, ", book.title);
    for page in pages {
        println!("\t{:?}", page.content);
    }
}

fn print_books_and_pages(books_with_pages: &Vec<BookWithPages>) {
    for book_with_pages in books_with_pages {
        print_book_and_pages(&book_with_pages.book, &book_with_pages.pages);
    }
}
fn main() {
    let args = Args::parse();
    let mut connection = orm_module::establish_connection();
    if let Some(name) = &args.book_name {
        let book = books::table
            .filter(books::title.eq(name))
            .first::<Book>(&mut connection)
            .unwrap();
        let pages = Page::belonging_to(&book)
            .load::<Page>(&mut connection)
            .unwrap();
        print_book_and_pages(&book, &pages);
    } else {
        let all_books = books::table
            .load::<Book>(&mut connection)
            .expect("Failed to load books");
        let pages = Page::belonging_to(&all_books)
            .load::<Page>(&mut connection)
            .expect("Failed to load pages");

        let pages_per_book = pages
            .grouped_by(&all_books)
            .into_iter()
            .zip(all_books)
            .map(|(pages, book)| BookWithPages { book, pages })
            .collect::<Vec<BookWithPages>>();
        print_books_and_pages(&pages_per_book);
    }
}
