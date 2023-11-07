use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Artifact {
    id: u64,
    adventure: u64,
    status: String,
    kind: String,
}

impl Artifact {
    pub fn new (id: u64, adventure: u64, status: String, kind: String) -> Self {
        Artifact { id, adventure, status, kind }
    }

    pub fn identify(&mut self) -> Result<(), ()> {
        if self.status == "Unidentified" {
            self.status = "Identified".to_string();
            Ok(())
        } else {
            Err(())
        }
    }
}
