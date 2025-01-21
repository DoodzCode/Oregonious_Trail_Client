use std::fmt::Debug;

use crate::structs::game_state::GameState;

pub fn status_report(game_data: &mut GameState) {
    println!("Status Report: ");
    println!("{}", game_data);

    println!();
    println!();
}
