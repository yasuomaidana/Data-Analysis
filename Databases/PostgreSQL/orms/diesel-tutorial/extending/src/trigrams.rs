use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about = "Create a book", long_about = None)]
struct Args {
    #[arg(help = "Text to search")]
    text_input: String,
}
fn main() {

}
