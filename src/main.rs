mod utils;
mod structs;
mod commands;

use crate::structs::Client;

fn main() {
    println!("Welcome to the game!");
    
    Client::start();
}
