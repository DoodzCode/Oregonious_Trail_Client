

use crate::structs::game_data;

pub fn status_report() {
    println!("Status Report: ");
    println!("{:?}", game_data);

    println!();
    println!();
}