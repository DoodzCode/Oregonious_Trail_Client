use std::collections::HashMap;
use std::io::Write;
// use tokio::io::AsyncWriteExt;
use crate::server::{self, wait_for_players};
use std::sync::{Arc, Mutex};


use crate::processors::{
    report_processor::status_report,
    conditions_processor::cycle_conditions
};
use crate::controllers::decision_controller::{
    self, 
    party_to_delay, 
    party_to_proceed
};
use crate::server::{PlayerCollection, ServerStatus};
use crate::structs::game_state::GameState;
use crate::utils::{
    d20, 
    save_to_file,
    load_game_from_file,
    get_input,
    line_break
};



pub fn game_loop(number_of_players: u16, server_status: &Arc<Mutex<ServerStatus>>) {

    
    
    // startup
    println!();
    println!("setup:");
    line_break();
    
    // Unlock the mutex and get access to the server status across threads
    let mut server_status_lock = server_status.lock().unwrap();
    // Dereference the mutex and reassign the value to the server status
    *server_status_lock = ServerStatus::WaitingForHost;
    drop(server_status_lock);


    let mut players: PlayerCollection = wait_for_players(number_of_players, 5000);

    for (addr, _) in players.iter() {
        println!("[PLAYER] {} is ready!", addr);
    }



    let mut game_state: GameState = load_game_from_file("src/config/game_state.json").expect("Failed to load game data");
    status_report(&mut game_state);
    println!("{:#?}", &game_state);


    // These lines could be moved to the Sever struct?
    server_status_lock = server_status.lock().unwrap();
    *server_status_lock = ServerStatus::Active;
    drop(server_status_lock);

    // main loop
    loop {
        if game_state.game_date.week_number > game_state.g_duration - 1 { break; }

        // Broadcast message to all players
        for (_, player) in players.iter_mut() {
            let _ = player.write_all("Test Message".as_bytes());
            // let _ = player.write_all(b"Test Message.").await;
        }
        // send_to_all_players(&players, "Test Message.").await;
        
        //* conditions_processor -  cycle conditions
        cycle_conditions(&mut game_state);

        //* decision_controller - user prompt for commands
        //* actions_processor - cycle actions    
        //* decision_controller - user prompt - go or no go.

        decision_controller::captains_orders(&mut game_state);

        /*
        for party in &mut game_state.parties {
            print!("{:?} do you want to 1. proceed or 2. delay?", party.name);
            let cmd: String = get_input();     
            let party_name = party.name.clone();
            match cmd.as_str() {
                "1" => party_to_proceed(party), 
                
                "2" => party_to_delay(party),

                _ => println!("Invalid Response")
            } 
            scores.insert(party_name, party.position);
        }   
        */

        // decision_controller();
        // Global Report

        println!("");
        for (key, value) in game_state.score.clone() {
            println!("{}: {}", key, value);
        }
        line_break();    
    }
    // shutdown

}