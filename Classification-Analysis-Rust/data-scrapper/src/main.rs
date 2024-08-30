use scraper::{Html, Selector};
use data_scrapper::remove_using_regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Book {
    title: String,
    price: String,
}

fn main() {
    let content = reqwest::blocking::get("https://books.toscrape.com/catalogue/page-1.html")
        .expect("Failed to send request");
    let document = Html::parse_document(&content.text().expect("Failed to parse response"));

    let book_selector = Selector::parse(".product_pod").unwrap();
    let books = document.select(&book_selector);
    for book in books {
        let title = book.select(&Selector::parse("h3 a").unwrap()).next().unwrap().inner_html();
        let price = book.select(&Selector::parse(".price_color").unwrap()).next().unwrap().inner_html();
        let book = Book {
            title: remove_using_regex(&title, r"[^a-zA-Z0-9 ]", ""),
            price: remove_using_regex(&price, r"[^0-9.]", ""),
        };
        println!("{:?}", book);
    }

}
