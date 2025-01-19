#[derive(Debug)]
pub struct Party {
    pub name: String,
    position: u16,
    pub head_count: u16,
    // pub wagons: Vec<Wagon>,
}

impl Party {
    pub fn create(name: &str) -> Party {
        Party {
            name: String::from(name),
            position: 0,
            head_count: 140,
        }
    }

    pub fn increment_position(&self, distance: u16) -> u16 {
        self.position + distance
    }

    pub fn decrement_position(&self, distance: u16) -> u16 {
        self.position - distance
    }

    pub fn increment_head_count(&self, amount: u16) -> u16 {
        self.head_count + amount
    }

    pub fn decrement_head_count(&self, amount: u16) -> u16 {
        self.head_count - amount
    }

    pub fn give_position(&self) -> &u16 {
        &self.position
    }
}
