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

    let mut legs: Vec<Leg> = Vec::new();

    legs.push(Leg{
        name: String::from("Leg-1"),
        destination: String::from("Kansas River crossing"),
        distance: 150,
        guide_note: String::from("West by north about 5 or 6 days to Kansas River crossing."),
    });

    legs.push(Leg{
        name: String::from("Leg-2"),
        destination: String::from("Platte River"),
        distance: 150,
        guide_note: String::from("Northwest, about 5 days to the Platte River."),
    });

    legs.push(Leg{
        name: String::from("Leg-3"),
        destination: String::from("Fort Kearney"),
        distance: 150,
        guide_note: String::from("Continue up south side of the Platte to Fort Kearney"),
    });

    legs.push(Leg{
        name: String::from("Leg-4"),
        destination: String::from("Fort Laramie"),
        distance: 150,
        guide_note: String::from("Follow the north fork of the Platte about 8 days to Fort Laramie."),
    });

    for leg in &legs { println!("ref {}, destination {}, distance {}, guide_note {}", leg.name, leg.destination, leg.distance, leg.guide_note );}

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






