// Anything that handles inputs from various sources?
// Listening for inputs and directing inputs to appropriate places

/// collect initial data to start game
pub fn get_initial_data() {
  println!(how many players );

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


