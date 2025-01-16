
use crate::structs::game_data::Game_Data;

   // Update Game Date
    // let current_date = start_date + (i * 7);
    // let current_date = start_date + "00".to_string();

pub fn cycle_conditions(game_data: &mut Game_Data) {

    game_data.week_number += 1;
    println!("Week # {}", game_data.week_number);

}
