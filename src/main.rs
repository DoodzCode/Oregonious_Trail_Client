mod utils;
mod client;
mod commands;
mod input_handler;

use crate::client::Client;

fn main() {
    println!("Welcome to the game!");
    Client::start();
}
