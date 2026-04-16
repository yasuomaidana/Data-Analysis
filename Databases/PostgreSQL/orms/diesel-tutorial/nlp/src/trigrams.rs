use clap::Parser;
use diesel::{QueryDsl, RunQueryDsl};
use orm_module::establish_connection;
use orm_module::model::sentence::Sentence;
use orm_module::schema::sentences_schema::sentences as sentences_schema;

use pgtrgm::prelude::TrgmExpressionMethods;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // see https://crates.io/crates/pgtrgm
    word: String,
}

fn main() {
    let cli = Cli::parse();
    let mut conn = establish_connection();
    let word = cli.word;
    let sentences = sentences_schema::table
        .filter(sentences_schema::text.trgm_word_similar_to(&word))
        .load::<Sentence>(&mut conn);
    match sentences {
        Ok(sentences) => {
            if sentences.is_empty() {
                println!("No sentences found similar to '{}'", word);
                return;
            }
            println!("Sentences similar to '{}':", word);
            for sentence in sentences {
                println!("{}", sentence.text);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
