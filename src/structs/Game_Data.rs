use crate::Biome;

/// Game_Data - conditions (states) that are not influenced by the conditions of the trail (biomes, segments), or the parties (wagons, people, animals). 
pub struct Game_Data{
    pub game_length: u8,
    pub week_number: u8,
    pub game_date: GameDate,
    pub biomes: Biome,

}

impl Game_Data {
    pub fn create_test() -> Game_Data {
        Game_Data{ 
            game_length: 26,
            week_number: 0,
            game_date: GameDate{
                week_number: 0, month: String::from("April"),
                biomes: Biome { },
        }

        }
    }
}

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

