mod structs;
mod processors;

extern crate chrono;

use structs::{
    game_data::Game_Data,
    // biome::Biome,
    // leg::Leg,
    // location::Location,
    // party::Party,
};
use processors::report_processor::status_report;
use processors::conditions_processor::cycle_conditions;
use processors::game_generator::{generate_game, generate_legs};

//TODO come back to the question of do we need territories to be separate?
// struct Territory {
//     name: String
// }

fn main() {

    // startup
    println!("-----------------------------------------------------------------------");
    println!("SETUP");
    println!("-----------------------------------------------------------------------");
    println!(); 


    let mut game_data: Game_Data = Game_Data::create_game();
    let mut game_data: generate_game()

    status_report(&mut game_data);

    // main loop
    loop {
        if game_data.game_date.week_number > game_data.game_length - 1  {
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

    // fn game_loop (game_cycle: u32) -> u32 {
    //    cycle_conditions(structs::game_data);
    //    game_cycle - 1
    // }

    // shutdown
    


    
}





/*
round
    player turn
*/






