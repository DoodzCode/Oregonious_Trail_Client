mod controllers;
mod engine;
mod processors;
mod server;
mod structs;
mod utils;

use server::{wait_for_players, PlayerCollection};
use engine::game_loop;
use utils::prompt_user;



#[tokio::main]
async fn main() {
    println!("Welcome to the game server!");

    let response = prompt_user("How many players?");
    let number_of_players: u16 = response.parse().expect("Failed to parse string to u16");

    let players: PlayerCollection = wait_for_players(number_of_players, 5000).await;

    for (addr, _) in players.iter() {
        println!("[PLAYER] {} is ready!", addr);
    }

    game_loop();
}
