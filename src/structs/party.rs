
#[derive(Debug)]
pub struct Party {
  pub name: String,
  pub position: u16, 
  pub head_count: u16,
}

impl Party {
  pub fn create(name: &str) -> Party {
    Party {
      name: String::from(name),
      position: 0,
      head_count: 140,
    }
  }
  pub fn 
}


