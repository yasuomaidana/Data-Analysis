use std::fs::File;
use std::io::Write;
use scraper::{Html, Selector};
use data_scrapper::remove_using_regex;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Book {
    title: String,
    price: String,
}

const ALPHANUMERIC_PATTERN: &str = r"[^a-zA-Z0-9 ]";

fn main() {
    let content = reqwest::blocking::get("https://books.toscrape.com/catalogue/page-1.html")
        .expect("Failed to send request");
    let document = Html::parse_document(&content.text().expect("Failed to parse response"));

    let book_selector = Selector::parse(".product_pod").unwrap();
    let books = document.select(&book_selector)
        .filter(|book| {
            let availability_selector = Selector::parse(".availability").unwrap();
            let availability_text = book.select(&availability_selector).next().unwrap().inner_html();
            availability_text.contains("In stock")
        })
        .map(|book| {
        let title_selector = Selector::parse("h3 a").unwrap();
        let price_selector = Selector::parse(".price_color").unwrap();

        let title = book.select(&title_selector).next().unwrap().inner_html();
        let price = book.select(&price_selector).next().unwrap().inner_html();

        let cleaned_title = remove_using_regex(&title, ALPHANUMERIC_PATTERN, "");
        let cleaned_price = remove_using_regex(&price, ALPHANUMERIC_PATTERN, "");

        Book {
            title: cleaned_title,
            price: cleaned_price,
        }
    }).collect::<Vec<Book>>();

    let file_path = "books.txt";
    let mut file = File::create(file_path).unwrap();
    for book in books {
        let title = book.title;
        let price = book.price;
        let entry = format!("{}, {}\n", title, price);
        file.write_all(entry.as_bytes()).unwrap();
    }
}