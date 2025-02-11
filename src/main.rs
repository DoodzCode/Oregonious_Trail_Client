mod utils;
mod structs;

use crate::structs::Client;

fn main() {
    println!("Welcome to the game!");
    
    Client::start();
}
