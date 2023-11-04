use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Adventurer {
    id: u64,
    name: String,
    surname: String,
    level: u32
}

impl Adventurer {
    pub fn new(id: u64, name: String, surname: String, level: u32) -> Self {
        Adventurer { id, name, surname, level}
    }
}
