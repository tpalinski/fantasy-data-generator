use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Participation {
    adventurer: u64,
    adventure: u64,
}

impl Participation {
    pub fn new(adventurer: u64, adventure: u64) -> Self {
        Participation { adventurer, adventure }
    }
}
