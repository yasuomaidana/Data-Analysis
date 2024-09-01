use regex::Regex;

pub fn remove_using_regex(text: &str, pattern: &str, new_to: &str) -> String {
    let re = Regex::new(pattern).unwrap();
    re.replace_all(text, new_to).to_string()
}