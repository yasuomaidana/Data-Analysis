use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use scraper::{Html, Selector};
use data_scrapper::remove_using_regex;
use serde::{Deserialize, Serialize};
use termion::{color};


#[derive(Debug, Deserialize, Serialize)]
struct Book {
    title: String,
    price: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    const URL: &str = "https://books.toscrape.com/catalogue/page-1.html";
    const ALPHANUMERIC_PATTERN: &str = r"[^a-zA-Z0-9 ]";

    let content = reqwest::blocking::get(URL)
        .expect("Failed to send request");
    let document = Html::parse_document(&content.text().expect("Failed to parse response"));

    let book_selector = Selector::parse(".product_pod").unwrap();
    let books: Vec<Book> = document.select(&book_selector)
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

            Book { title: cleaned_title, price}
        })
        .collect();

    let json_data = serde_json::json!(books);

    let file_path = "books.json";
    let mut file = File::create(file_path)?;
    let json_string = serde_json::to_string(&json_data)?;
    file.write_all(json_string.as_bytes())?;

    print_books_from_json(file_path);
    fs::remove_file(file_path)?;

    Ok(())
}

fn print_books_from_json(filename: &str) {
    let data = std::fs::read_to_string(filename).unwrap();
    let books: Vec<Book> = serde_json::from_str(&data).unwrap();

    for book in books {
        let title_green = format!("{}{}", color::Fg(color::Green), book.title);
        let price_orange = format!("{}{}", color::Fg(color::LightRed), book.price);
        println!("{}: {}", title_green, price_orange);
    }
}