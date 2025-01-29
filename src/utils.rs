use rand::Rng;

pub fn d20() -> u8 {
  let mut rng = rand::thread_rng();
  rng.gen_range(1..=20)
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_d20() {
    let result = d20();
    assert!(result >= 1 && result <= 20);
  }
}