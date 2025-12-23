use std::io::{self, Write};
use std::str::FromStr;

pub fn get_arg_or_prompt<T: FromStr>(args: &Vec<String>, idx: usize, prompt: &str) -> T {
    if let Some(s) = args.get(idx) {
        let s = s.trim();
        if !s.is_empty() {
            if let Ok(val) = s.parse::<T>() {
                return val;
            }
        }
    }

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input
        .trim()
        .parse::<T>()
        .unwrap_or_else(|_| panic!("Failed to parse input"))
}

pub fn get_arg_or_default<T: FromStr>(args: &Vec<String>, idx: usize, default: T) -> T {
    args.get(idx)
        .and_then(|s| s.trim().parse::<T>().ok())
        .unwrap_or(default)
}