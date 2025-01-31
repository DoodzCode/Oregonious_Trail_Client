use core::fmt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Biome {
    pub name: String,
    pub humidity_factor: u8,
    pub temperature_factor: u8,
    pub pressure_factor: u8,
    // pub b_type: BiomeType,
}

impl fmt::Display for Biome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Biome.name: {} \n", self.name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
enum BiomeType {
    Grasslands,
    Forest,
    Desert,
    Mountain,
    Canyon,
}

impl BiomeType {
    pub fn base_chance_of_rain(&self) -> u8 {
        match self {
            BiomeType::Grasslands => 50,
            BiomeType::Forest => 60,
            BiomeType::Desert => 10,
            BiomeType::Mountain => 40,
            BiomeType::Canyon => 30,
        }
    }
}