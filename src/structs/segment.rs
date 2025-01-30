use core::fmt;
use serde::{Serialize, Deserialize};

/// struct for Segments (segments) of the trail
#[derive(Serialize, Deserialize, Debug)]
pub struct Segment {
    pub name: String,
    pub distance: i32,      // 80
    pub guide_note: String, // "west by north to Kansas River Crossing"
}

impl Segment {
    pub fn create(name: &str, distance: i32, guide_note: String) -> Segment {
        Segment {
            name: String::from(name),
            distance,
            guide_note,
        }
    }

    //TODO add a generator function
}

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Segment.name: {} \n Segment.distance: {} \n guide_note: {} \n", self.name, self.distance, self.guide_note)
    }
}
