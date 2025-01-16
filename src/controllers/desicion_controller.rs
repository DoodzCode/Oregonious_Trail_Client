// collect data to present
// prompt for player input


pub fn get_input() -> String {
  let mut r_input: String = String::new();
  std::io::stdin().read_line(&mut r_input).unwrap();
  let input: &str = r_input.trim();
  String::from(input).to_lowercase()
}
