mod processors;
mod structs;
mod utils;

extern crate chrono;

use processors::conditions_processor::cycle_conditions;
use processors::report_processor::status_report;
use structs::game_state::GameState;

use utils::{d20, save_to_file, load_game_from_file};

//TODO come back to the question of do we need territories to be separate?

fn main() {
    // startup
    println!("-----------------------------------------------------------------------");
    println!("SETUP");
    println!("-----------------------------------------------------------------------");
    println!();

    let mut game_data: GameState = load_game_from_file("src/config/game_data.json").expect("Failed to load game data");

    status_report(&mut game_data);

    println!("{:?}", &game_data);

    // main loop
    loop {
        if game_data.game_date.week_number > game_data.game_length - 1 {
            break;
        }
        cycle_conditions(&mut game_data);
        // user prompt
        //* decision_controller();
        //cycle actions
        //* actions_processor();
        // user prompt - go or no go.
        //* decision_controller();
        // Global Report
    }

    // shutdown
}

/*
round
    player turn
*/
