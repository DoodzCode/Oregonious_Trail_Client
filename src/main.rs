mod controllers;
mod engine;
mod processors;
mod server;
mod structs;
mod utils;

use server::wait_for_players;
use engine::game_loop;



#[tokio::main]
async fn main() {
    let players = wait_for_players(5000).await;

    for (addr, _) in players.iter() {
        println!("[PLAYER] {} is ready!", addr);
    }

    game_loop();
}
