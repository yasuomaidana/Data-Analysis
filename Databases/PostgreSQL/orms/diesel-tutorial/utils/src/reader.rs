use std::fmt::Debug;
use std::io::{self, Write};
use std::str::FromStr;

#[derive(Debug)]
pub enum ReaderError<E: Debug> {
    ParseError(E),
    IoError(io::Error),
}

pub fn get_arg_or_prompt<T: FromStr>(args: &Vec<String>, idx: usize, prompt: &str) -> T
where
    T::Err: Debug,
{
    match get_arg::<T>(&args, idx) {
        Ok(Some(s)) => {
            return s;
        }
        Ok(None) => {}
        Err(ReaderError::ParseError(e)) => {
            println!("Parse error: {:?}\n Try new input", e);
        }
        Err(ReaderError::IoError(e)) => {
            println!("Io error: {:?}\n Try new input", e);
        }
    }

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    read_line::<T>().unwrap_or_else(|e| match e {
        ReaderError::ParseError(pe) => {
            panic!("Parsing error: {:?}", pe)
        }
        ReaderError::IoError(ie) => {
            panic!("IoError error: {:?}", ie)
        }
    })
}

pub fn read_line<T: FromStr>() -> Result<T, ReaderError<T::Err>>
where
    T::Err: Debug,
{
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .map_err(ReaderError::IoError)?;
    let s = input.trim();
    s.parse::<T>().map_err(|e| ReaderError::ParseError(e))
}

pub fn get_arg_or_default<T: FromStr>(args: &Vec<String>, idx: usize, default: T) -> T {
    args.get(idx)
        .and_then(|s| s.trim().parse::<T>().ok())
        .unwrap_or(default)
}

pub fn get_arg<T: FromStr>(args: &Vec<String>, idx: usize) -> Result<Option<T>, ReaderError<T::Err>>
where
    T::Err: Debug,
{
    match args.get(idx) {
        Some(s) => {
            let trimmed = s.trim();
            if trimmed.is_empty() {
                Ok(None)
            } else {
                trimmed
                    .parse::<T>()
                    .map(Some)
                    .map_err(|e| ReaderError::ParseError(e))
            }
        }
        None => Ok(None),
    }
}
