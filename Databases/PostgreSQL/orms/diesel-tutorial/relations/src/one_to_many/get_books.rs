use clap::Parser;
use diesel::{BelongingToDsl, ExpressionMethods};
use diesel::{GroupedBy, QueryDsl};
use diesel::{RunQueryDsl, SelectableHelper};
use orm_module::model::book::{Book, BookWithPages, CompleteBook, Page};
use orm_module::model::user::UserEmail;
use orm_module::schema::book_schema::books;
use orm_module::schema::user_schema::users;

#[derive(Parser, Debug)]
#[command(author, version, about = "Create a book", long_about = None)]
struct Args {
    #[arg(value_name = "NAME", help = "Book name to search")]
    book_name: Option<String>,
    #[arg(value_name = "user", help = "Book name to search", short = 'u')]
    user_name: Option<String>,
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

fn handle_book_name(name: &String, connection: &mut diesel::PgConnection) {
    let book = books::table
        .filter(books::title.eq(name))
        .first::<Book>(connection)
        .unwrap();
    let pages = Page::belonging_to(&book).load::<Page>(connection).unwrap();
    print_book_and_pages(&book, &pages)
}

fn handle_username(username: &String, connection: &mut diesel::PgConnection) {
    let author_pairs = books::table
        .inner_join(users::table)
        .filter(users::name.eq(username))
        .select((Book::as_select(), UserEmail::as_select()))
        .load::<(Book, UserEmail)>(connection)
        .expect("Failed to load author's books");

    if author_pairs.len() > 0 {
        let complete_books = author_pairs
            .into_iter()
            .map(|(book, author)| {
                let pages = Page::belonging_to(&book).load::<Page>(connection).unwrap();
                CompleteBook {
                    author,
                    book: BookWithPages { book, pages },
                }
            })
            .collect::<Vec<CompleteBook>>();

        for complete_book in complete_books {
            println!("{:#?}", complete_book.author);
            print_book_and_pages(&complete_book.book.book, &complete_book.book.pages);
        }
    } else {
        println!("No books found for user: {} showing all books", username);
        books::table
            .left_join(users::table)
            .select((Book::as_select(), Option::<UserEmail>::as_select()))
            .load::<(Book, Option<UserEmail>)>(connection)
            .unwrap_or_else(|_| Vec::new())
            .iter()
            .for_each(|(book, user_email)| {
                let author = match user_email {
                    None => "Anonymous".to_string(),
                    Some(user) => user.name.to_string(),
                };
                println!("Book: {:?} by {}", book.title, author);
            });
    }
}

fn main() {
    let args = Args::parse();
    let mut connection = orm_module::establish_connection();
    if let Some(name) = &args.book_name {
        handle_book_name(name, &mut connection);
    } else if let Some(username) = &args.user_name {
        handle_username(username, &mut connection);
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
