use diesel::dsl::count_star;
use diesel::insert_into;
use diesel::prelude::*;
use orm_module::establish_connection;
use orm_module::schema::jtrack_schema::jtrack as jtrack_table;
use orm_module::schema::jtrack_schema::jtrack::dsl::jtrack;
use serde_json::Value;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = establish_connection();
    let count: i64 = jtrack
        .select(count_star())
        .get_result(&mut connection)
        .expect("Error counting jtrack entries");
    println!("There are {} jtrack entries", count);

    if count <= 0 {
        // Fetch JSON list from remote URL (blocking)
        let resp = reqwest::blocking::get("https://www.pg4e.com/code/library.jstxt")?;
        let text = resp.text()?;

        // parse into a list of JSON values (Vec<Value>) using a helper
        let values = parse_to_values(&text);
        println!("Parsed {} JSON value(s)", values.len());
        // Convert parsed JSON values into insertable records for the jtrack table
        #[derive(diesel::Insertable)]
        #[diesel(table_name = jtrack_table)]
        struct NewJtrack {
            body: Option<Value>,
        }

        let new_rows: Vec<NewJtrack> = values
            .into_iter()
            .map(|v| NewJtrack { body: Some(v) })
            .collect();
        insert_into(jtrack)
            .values(&new_rows)
            .execute(&mut connection)?;
        // You can now use `values` (Vec<Value>) as needed
    }

    Ok(())
}

// Try several parsing strategies and print any parsed JSON values.
// Parse text into a Vec<Value> using common strategies.
// Strategies (in order):
// 1) Parse as single JSON value (if it's an array, return its elements).
// 2) Extract JSON starting at first '[' or '{' (handles JS wrappers like `var x = ...;`).
// 3) NDJSON: parse each non-empty line as JSON.
// 4) Stream-deserialize multiple concatenated JSON values.
fn parse_to_values(text: &str) -> Vec<Value> {
    // 1) Single JSON value
    if let Ok(val) = serde_json::from_str::<Value>(text) {
        return match val {
            Value::Array(arr) => arr,
            other => vec![other],
        };
    }

    // 2) Try extracting from a JS wrapper like "var lib = [...] ;"
    if let Some(start) = text.find(|c| c == '[' || c == '{') {
        let candidate = text[start..].trim();
        let candidate = candidate.trim_end_matches(|c: char| c == ';' || c.is_whitespace());
        if let Ok(val) = serde_json::from_str::<Value>(candidate) {
            return match val {
                Value::Array(arr) => arr,
                other => vec![other],
            };
        }
    }

    // 3) NDJSON: parse each non-empty line
    let mut results = Vec::new();
    for line in text.lines() {
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<Value>(s) {
            results.push(v);
        }
    }
    if !results.is_empty() {
        return results;
    }

    // 4) Stream-deserialize multiple concatenated JSON values
    if let Ok(values) = serde_json::Deserializer::from_str(text)
        .into_iter::<Value>()
        .collect::<Result<Vec<_>, _>>()
    {
        if !values.is_empty() {
            return values;
        }
    }

    // Nothing parsed
    Vec::new()
}
