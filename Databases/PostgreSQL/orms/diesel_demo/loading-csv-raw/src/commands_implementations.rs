use std::fs::File;
use std::path::Path;
use reqwest::blocking;

pub fn download(url: &String){
    if Path::new("library.csv").exists() { 
        println!("File already exists, skipping download.");
        return;
    } else {
        println!("Downloading...");
        let response = blocking::get(url).expect("Could not fetch URL");
        if !response.status().is_success() {
            panic!("Failed to download file: {}", response.status());
        }
        let mut file = File::create("library.csv").expect("Failed to create file");
        std::io::copy(&mut response.bytes().unwrap().as_ref(), &mut file).expect("Failed to write to file");
        println!("Download complete.");
    }
}