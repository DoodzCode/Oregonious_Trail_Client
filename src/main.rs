mod structs;
mod conditions_processor;

extern crate chrono;
use structs::game_data::Game_Data;
use conditions_processor::cycle_conditions;

struct Territory {
    name: String
}

struct Biome {
    name: String
}

struct Location {
    name: String
}

#[derive(Debug)]
struct Leg {
    name: String,
    destination: String,
    distance: i32,  // 80
    guide_note: String, // "west by north to Kansas River Crossing"
}

struct Game {
    start_date: String,
    legs: Vec<Leg>,
    // locations: Locations,

}


fn main() {

    // startup
    println!("STARTUP");

  

    // let mut game: Game = Game::new();

    let mut game_data: Game_Data = Game_Data::create_test();
    println!("Week # {}", game_data.week_number);

    // main loop
    loop {
        if game_data.week_number > game_data.game_length - 1  {
            break;
        }
        cycle_conditions(&mut game_data);
        // user prompt 
        //cycle actions
        // user prompt - go or no go.
        
    }


    // shutdown
    
}

/*
round
    player turn
*/






