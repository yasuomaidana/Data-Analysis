use std::fs::File;
use std::io::Write;
use scraper::{Html, Selector};
use data_scrapper::remove_using_regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
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

            let title_element = book.select(&title_selector).next().unwrap();
            let title = title_element.attr("title").unwrap_or_default();

            let price = book.select(&price_selector).next().unwrap().inner_html();

            let cleaned_title = remove_using_regex(&title, ALPHANUMERIC_PATTERN, "");

            Book {
                title: cleaned_title,
                price
            }
        }).collect::<Vec<Book>>();

    let json_books = serde_json::to_string(&books).unwrap();

    let file_path = "books.json";
    let mut file = File::create(file_path).unwrap();
    file.write_all(json_books.as_bytes()).unwrap();

    println!("Books data saved to {}", file_path);
}