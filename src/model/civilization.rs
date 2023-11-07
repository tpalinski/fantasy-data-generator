use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Civilization {
    civilization_name: String,
    race: String,
    period: i32,
    religion: String
} 

impl Civilization {
    pub fn new(civilization_name: String, race: String, period: i32, religion: String) -> Self {
        Civilization { civilization_name, race, period, religion}
    }

    pub fn get_name(&self) -> String {
        self.civilization_name.to_string()
    }
}
