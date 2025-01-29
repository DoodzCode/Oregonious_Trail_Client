mod processors;
mod structs;
mod utils;

extern crate chrono;

use processors::conditions_processor::cycle_conditions;
use processors::report_processor::status_report;
use structs::{
    game_state::GameState,
    // biome::Biome,
    // leg::Leg,
    // location::Location,
    // party::Party,
};

//TODO come back to the question of do we need territories to be separate?

fn main() {
    // startup
    println!("-----------------------------------------------------------------------");
    println!("SETUP");
    println!("-----------------------------------------------------------------------");
    println!();

    // let mut game_data: Game_Data = Game_Data::create_game();
    let mut game_data: GameState = GameState::create_game();

    status_report(&mut game_data);

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
