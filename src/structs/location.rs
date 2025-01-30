use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    name: String,
    elevation: u32,
    mile_marker: u32,
}
