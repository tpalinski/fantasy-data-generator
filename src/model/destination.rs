use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Destination {
    id: u64,
    name: String,
    coordinates: String,
    difficulty: u32,
    civilization: String,
}

impl Destination {
    pub fn new(id: u64, name: String, coordinates: String, difficulty: u32, civilization: String) -> Self {
        Destination { id, name, coordinates, difficulty, civilization}
    }
}
