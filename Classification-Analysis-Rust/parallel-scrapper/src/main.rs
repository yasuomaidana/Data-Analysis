use regex::Regex;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct ScrapedData {
    text: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read the list of URLs from a file
    let urls = read_urls_from_file("urls.txt").unwrap_or_else(|err| {
        eprintln!("Error reading URLs: {}\n\tretrieving default", err);
        vec!["https://www.rust-lang.org".to_string()]
    });

    // Read the selectors from a file
    let selectors = read_selectors_from_file("selectors.txt").unwrap_or_else(|err| {
        eprintln!("Error reading selectors: {}\t\n\tretrieving default", err);
        vec![vec!["p".to_string()]]
    });

    // Shared map to store word counts across all threads
    let word_counts = Arc::new(Mutex::new(HashMap::new()));

    // Create a vector to hold the join handles for the threads
    let mut handles = vec![];

    // Process each URL in parallel
    for url in urls {
        let word_counts = Arc::clone(&word_counts);
        let selectors = selectors.clone();
        let handle = thread::spawn(move || match process_url(&url, &selectors) {
            Ok(scraped_data) => {
                let mut word_counts = word_counts.lock().unwrap();
                count_words(&scraped_data.text, &mut word_counts);
            }
            Err(err) => eprintln!("Error processing {}: {}", url, err),
        });
        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Print the top 10 words
    let word_counts = word_counts.lock().unwrap();
    print_top_words(&word_counts, 10);

    // Store the extracted information
    store_data(&word_counts, "output.txt")?;

    Ok(())
}

fn read_urls_from_file(filename: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut urls = Vec::new();
    for line in reader.lines() {
        urls.push(line?);
    }
    Ok(urls)
}

fn read_selectors_from_file(filename: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut selectors = Vec::new();
    for line in reader.lines() {
        let selector_str = line?;
        let parts: Vec<String> = selector_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        selectors.push(parts);
    }
    Ok(selectors)
}

fn process_url(url: &str, selectors: &[Vec<String>]) -> Result<ScrapedData, Box<dyn Error>> {
    let response = reqwest::blocking::get(url)?;
    let body = response.text()?;
    let document = Html::parse_document(&body);

    let mut extracted_text = String::new();
    for selector_parts in selectors {
        if selector_parts.len() == 1 {
            // Single-level selector
            let selector = Selector::parse(&selector_parts[0]).unwrap();
            for element in document.select(&selector) {
                extracted_text.push_str(&element.inner_html());
                extracted_text.push(' ');
            }
        } else if selector_parts.len() == 2 {
            // Two-level selector
            let parent_selector = Selector::parse(&selector_parts[0]).unwrap();
            let child_selector = Selector::parse(&selector_parts[1]).unwrap();
            for parent_element in document.select(&parent_selector) {
                for child_element in parent_element.select(&child_selector) {
                    extracted_text.push_str(&child_element.inner_html());
                    extracted_text.push(' ');
                }
            }
        } else {
            return Err("Invalid selector format".into());
        }
    }

    Ok(ScrapedData {
        text: extracted_text,
    })
}

fn count_words(text: &str, word_counts: &mut HashMap<String, u32>) {
    let re = Regex::new(r"\b\w+\b").unwrap();
    for word in re.find_iter(text) {
        *word_counts.entry(word.as_str().to_lowercase()).or_insert(0) += 1;
    }
}

fn print_top_words(word_counts: &HashMap<String, u32>, n: usize) {
    let mut counts: Vec<_> = word_counts.iter().collect();
    counts.sort_by(|a, b| b.1.cmp(a.1));
    println!("Top {} words:", n);
    for (i, (word, count)) in counts.iter().enumerate().take(n) {
        println!("{}. {} ({})", i + 1, word, count);
    }
}

fn store_data(word_counts: &HashMap<String, u32>, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filename)?;
    for (word, count) in word_counts {
        file.write(format!("{},{}\n", word, count).as_bytes())?;
    }
    Ok(())
}
