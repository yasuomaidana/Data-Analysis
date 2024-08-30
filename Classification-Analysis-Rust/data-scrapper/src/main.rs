use rand::prelude::SliceRandom;
use scraper::{Html, Selector};
use rand::thread_rng;


fn main() {
    let response = reqwest::blocking::get("https://en.wikipedia.org/wiki/List_of_states_of_Mexico")
        .expect("Failed to send request");
    let html_content = response.text().unwrap();
    let html_document = Html::parse_document(&html_content);
    let table_selector = Selector::parse("table.wikitable tbody tr").unwrap();
    let html_table_rows = html_document.select(&table_selector);
    let state_selector = Selector::parse("td:first-child").unwrap();
    let city_selector = Selector::parse("td:nth-child(5)").unwrap();
    let capital_selector = Selector::parse("td:nth-child(4)").unwrap();
    let td_selector = Selector::parse("td").unwrap();
    let mut states = Vec::new();
    let mut capitals = Vec::new();
    let mut cities = Vec::new();

    for row in html_table_rows{
        let elements = row.select(&td_selector).count();
        if elements<9{
            continue;
        }
        let state = row.select(&state_selector).next().unwrap().text().next().unwrap();
        states.push(state);
        let capital = row.select(&capital_selector).next().unwrap().text().next().unwrap();
        capitals.push(capital);
        let city = if elements == 9 {
            capital
        }else{
            row.select(&city_selector).next().unwrap().text().next().unwrap()
        };
        cities.push(city);
    }
    let mut all_elements = Vec::new();
    all_elements.extend(cities);
    all_elements.extend(states);
    all_elements.extend(capitals);

    let mut rng = thread_rng();
    all_elements.shuffle(&mut rng);
    println!("{}", all_elements.join(", "))
}
