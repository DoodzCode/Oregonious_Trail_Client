mod engine;
mod controllers;
mod processors;
mod structs;
mod utils;
mod server;

extern crate chrono;
use engine::game_loop;

#[tokio::main]

async fn main() {
    let players = server::wait_for_players(5000).await;

    let players_lock = players.lock().await;
    for (addr, _) in players_lock.iter() {
        println!("[PLAYER] {} is ready!", addr);
    }
    game_loop();
}    

use tokio::time::{sleep, Duration};

use std::collections::HashMap;
use processors::conditions_processor::cycle_conditions;
use processors::report_processor::status_report;
use controllers::decision_controller::{self, party_to_delay, party_to_proceed};

use structs::{game_state::GameState, party};
use utils::{d20, save_to_file, load_game_from_file, get_input};

//TODO come back to the question of do we need territories to be separate?




fn game_loop() {
    let mut game_state: GameState = load_game_from_file("src/config/game_state.json").expect("Failed to load game data");
    status_report(&mut game_state);
    println!("{:#?}", &game_state);


    // main loop
    loop {
        if game_state.game_date.week_number > game_state.g_duration - 1 { break; }
        
        //* conditions_processor -  cycle conditions
        cycle_conditions(&mut game_state);

        //* decision_controller - user prompt for commands
        //* actions_processor - cycle actions    
        //* decision_controller - user prompt - go or no go.

        decision_controller::captains_orders(&mut game_state);

        println!("");
        for (key, value) in game_state.score.clone() {
            println!("{}: {}", key, value);
        }
        line_break();    
    }
}
