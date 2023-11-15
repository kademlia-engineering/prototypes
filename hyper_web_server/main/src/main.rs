/*
Last updated: 11-15-2023

Description: This crate defines the main thread of the blockchain node

Author: James Dean
*/
use server::run_server;
use config::load_config;

#[tokio::main]
async fn main() {
    let config = load_config("../config.json")
        .expect("Failed to load config");

    run_server(config.port).await;
}
