
#[derive(Debug)]
pub struct Party {
  pub name: String,
  pub position: u16, 
}

impl Party {
  pub fn create(name: &str, position: u16) -> Party {
    Party {
      name: String::from(name),
      position: position,
    }
  }
}


