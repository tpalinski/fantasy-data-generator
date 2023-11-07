use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Adventure {
    id: u64,
    facility: u64,
    destination: u64,
    date: String,
    deadline: String,
    status: String,
    cost: u64,
}

impl Adventure {
    pub fn new(id: u64, facility: u64, destination: u64, date: String, deadline: String, status: String, cost: u64) -> Self {
        Adventure { id, facility, destination, date, deadline, status, cost }
    }
}
