use crate::structs::game_data::{ActionType, Game_Data, Message};

// Update Game Date
// let current_date = start_date + (i * 7);
// let current_date = start_date + "00".to_string();

pub fn cycle_conditions(game_data: &mut Game_Data) {
    // calandar
    // game_data.week_number += 1;
    game_data.change_state(Message {
        action: ActionType::IncWeek,
    });

    println!("Week # {}", game_data.game_date.week_number);

    // biomes

    // determine which biomes have player in them

    // loop through active biomes

    // output: chance_of_snow = snow_factor();
    // input: d20
    // input: is date in winter

    // legs

    // cycle through the parties
    fn cycle_parties(party_count: u32) {}
}
