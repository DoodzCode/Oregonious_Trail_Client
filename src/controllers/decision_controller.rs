// Anything that handles inputs from various sources?
// Listening for inputs and directing inputs to appropriate places

use crate::structs::party::Party;

/// collect initial data to start game
pub fn get_initial_data() {
  // println!("how many players");

  // add player
  // name of party
  // manifest

}

// collect data to present

// prompt for player input

pub fn get_input() -> String {
  let mut r_input: String = String::new();
  std::io::stdin().read_line(&mut r_input).unwrap();
  let input: &str = r_input.trim();
  String::from(input).to_lowercase()
}

pub fn party_to_proceed(party: &mut Party) {
  println!("{:?} decides to proceed.", party.name);
  party.increment_position(80);
  // party.position
}

pub fn party_to_delay(party: &mut Party) {
  println!("{:?} decides to delay.", party.name)
}


