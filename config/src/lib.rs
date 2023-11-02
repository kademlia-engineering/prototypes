use serde::Deserialize;
use std::fs::File;
use std::io::Read;

// Define a struct to represent the JSON data structure
#[derive(Deserialize)]
pub struct Config {
    pub port: u16,
    pub jwt_key: String
}

// Function to read and parse the JSON file
pub fn read_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    // Open the JSON file
    let mut file: File = match File::open(file_path) {
        Ok(file) => file,
        Err(err) => return Err(Box::new(err)),
    };

    // Read the contents of the file into a String
    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),
        Err(err) => return Err(Box::new(err)),
    }

    // Parse the JSON data into a Config struct
    let config: Config = match serde_json::from_str(&contents) {
        Ok(config) => config,
        Err(err) => return Err(Box::new(err)),
    };

    Ok(config)
}

