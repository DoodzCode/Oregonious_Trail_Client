use core::fmt;

/// struct for legs (segments) of the trail
#[derive(Debug)]
pub struct Leg {
    pub name: String,
    pub distance: i32,      // 80
    pub guide_note: String, // "west by north to Kansas River Crossing"
}

impl Leg {
    pub fn create(name: &str, distance: i32, guide_note: String) -> Leg {
        Leg {
            name: String::from(name),
            distance,
            guide_note,
        }
    }

    //TODO add a generator function
}

impl fmt::Display for Leg {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Leg.name: {} \n Leg.distance: {} \n guide_note: {} \n", self.name, self.distance, self.guide_note)
    }
}
