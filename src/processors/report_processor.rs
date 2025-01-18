use crate::structs::game_data::Game_Data;



pub fn status_report(game_data: &mut Game_Data) {
    println!("Status Report: ");
    println!("{:?}", game_data);

    println!();
    println!();
}