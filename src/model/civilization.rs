use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Civilization {
    name: String,
    race: String,
    period: i32,
    religion: String
} 

impl Civilization {
    pub fn new(name: String, race: String, period: i32, religion: String) -> Self {
        Civilization { name, race, period, religion}
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}
