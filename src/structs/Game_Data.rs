pub struct Game_Data{
    pub game_length: u8,
    pub week_number: u8,
}

impl Game_Data {
    pub fn create_test() -> Game_Data {
        Game_Data{ 
            game_length: 26,
            week_number: 0,
        }
    }
}

