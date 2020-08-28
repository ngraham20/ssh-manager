
use json;
use std::fs;
use std::io;


/// Load JSON file into the program. The `tree` enum will be constructed
/// from the J
pub fn load_json() -> Result<json::JsonValue, io::Error> {
    
    let contents = fs::read_to_string("dat/connections.json")
        .expect("Failed to read file");

    let parsed = json::parse(&contents).unwrap();
    Ok(parsed)
}