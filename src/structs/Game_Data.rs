use crate::structs::{
    biome::Biome,
    leg::Leg,
    party::Party,
};
use crate::processors::game_generator::generate_legs;

/// Game_Data - conditions (states) that are not influenced by the conditions of the trail (biomes, segments), or the parties (wagons, people, animals). 
#[derive(Debug)]
pub struct Game_Data{
    pub start_date: String,
    pub game_length: u8,
    pub game_date: GameDate,
    pub biomes: Vec<Biome>,
    pub legs: Vec<Leg>,
    pub parties: Vec<Party>,

}


impl Game_Data {
    pub fn create_test() -> Game_Data {

        let mut test_data = Game_Data{ 
            start_date: String::from("April 15, 1841"),
            game_length: 26,
            game_date: GameDate{
                week_number: 0, month: String::from("April"),
            },
            biomes: Vec::new(),
            legs: generate_legs(),
            parties: Vec::new(),
        };

        test_data.biomes.push(Biome{ name: String::from("Biome Uno") });
        test_data

        }
    }

    #[derive(Debug)]
pub struct GameDate {
    pub week_number: u8,
    pub month: String,
}

impl GameDate {
    // game_data.game_data::increment_week();
    pub fn increment_week(&mut self) {
        self.week_number += 1;
    }
}

