use crate::structs::location::Location;

#[derive(Debug)]
pub struct Leg {
    pub name: String,
    // origin: Location,
    // destination: Location,
    pub distance: i32,  // 80
    pub guide_note: String, // "west by north to Kansas River Crossing"
}

impl Leg {
    pub fn create(name: &str, origin: Location, destination: Location, distance: i32, guide_note: String) -> Leg {
        Leg {
            name: String::from(name),
            // origin,
            // destination,
            distance,
            guide_note,
        }
    }

    //TODO add a generator function
}