mod controllers;
mod engine;
mod processors;
mod server;
mod structs;
mod utils;
mod client;

use client::client::game_client;

#[tokio::main]
async fn main() {
        game_client();
}