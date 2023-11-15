/*
Last updated: 11-15-2023

Description: This crate defines the configuration for the node

Author: James Dean
*/
use serde::{Deserialize, Serialize};
use std::fs;

pub const VERSION: &str = "0.1.0";

// This struct defines the configuration structure
#[derive(Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    // Add other configuration fields as needed
}

pub fn load_config(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string(file_path)?;
    let config: Config = serde_json::from_str(&config_str)?;
    Ok(config)
}
