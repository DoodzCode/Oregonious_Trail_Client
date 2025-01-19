use crate::structs::{
    biome::Biome,
    leg::Leg,
    party::Party,
};
use crate::processors::game_generator::{generate_legs, generate_parties};

/// Game_Data - conditions (states) that are not influenced by the conditions of the trail (biomes, segments), or the parties (wagons, people, animals). 
#[derive(Debug)]
pub struct Game_Data{
    pub start_date: String,
    pub game_length: u8,
    pub game_date: GameDate,
    pub biomes: Vec<Biome>,
    pub legs: Vec<Leg>,
    pub parties: Vec<Party>,
    pub score: Vec<Player_Score>,
}


impl Game_Data {
    pub fn create_game() -> Game_Data {

        let mut test_data = Game_Data { 
            start_date: String::from("April 15, 1842"),
            game_length: 26,
            game_date: GameDate {
                week_number: 0, month: String::from("April"),
            },
            biomes: Vec::new(),
            legs: generate_legs(),
            parties: generate_parties(),
            score: Vec::new(),
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

// score 
#[derive(Debug)]
pub struct Player_Score {
    pub party_name: String,
    pub position: u32,
    pub head_count: u32, 
}

impl Player_Score {
    pub fn increment_position(&mut self, inc: u32) {
        self.position += inc;
    }
    pub fn decrement_position(&mut self, dec: u32) {
        self.position -= dec;
    }
    pub fn increment_head_count(&mut self, inc: u32) {
        self.head_count += inc;
    }
    pub fn decrement_head_count(&mut self, dec: u32) {
        self.head_count -= dec;
    }
}