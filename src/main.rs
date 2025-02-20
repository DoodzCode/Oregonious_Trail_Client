mod utils;
mod client;
// mod commands;
// mod input_handler;
// mod orders;

use crate::client::Client;

fn main() {
    println!("Welcome to the game!");
    Client::start();
}
