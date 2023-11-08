use std::fs;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ConfigData {
    pub first_period: FirstPeriod,
    pub second_period: SecondPeriod,
}

#[derive(Serialize, Deserialize)]
struct FirstPeriod {
    pub adventurers: u64,
    pub destinations: u64,
    pub adventures: u64,
    pub participations: u64,
    pub artifacts: u64,
    pub date: String,
    pub start_date: String,
}

#[derive(Serialize, Deserialize)]
struct SecondPeriod {
    pub date: String,
    pub adventures: u64,
    pub participations: u64,
    pub artifacts: u64,
    pub adventurers: u64,
    pub identified_artifacts: u64,
}

impl ConfigData {
    pub fn new() -> Self {
        ConfigData { first_period: FirstPeriod { adventurers: 0, destinations: 0, adventures: 0, participations: 0, date: "01.01.1900".to_string(), artifacts: 0, start_date: "01.01.1900".to_string()}, second_period: SecondPeriod { date: "01.02.1900".to_string() , adventures: 0, participations: 0, artifacts: 0, adventurers: 0, identified_artifacts: 0} }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ConfigReader {
    config_path: String,
    config_data: ConfigData,
}

impl ConfigReader {
    pub fn new(path: &str) -> Self {
        ConfigReader { config_path: String::from(path), config_data: ConfigData::new() } }

    pub fn get_config(&mut self) {
       let contents = match fs::read_to_string(&self.config_path) {
            Ok(c) => c,
            Err(_) => {
                panic!("Error reading config file")
            }
        };
        let config_data: ConfigData = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                panic!("Error deserializing config file")
            }
        };
        self.config_data = config_data;
    }

    pub fn get_first_adventurers(&self) -> u64{
        self.config_data.first_period.adventurers
    }

    pub fn get_second_adventurers(&self) -> u64 {
        self.config_data.second_period.adventurers
    }

    pub fn get_first_destinations(&self) -> u64 {
        self.config_data.first_period.destinations
    }

    pub fn get_first_adventures(&self) -> u64 {
        self.config_data.first_period.adventures
    }

    pub fn get_second_adventures(&self) -> u64 {
        self.config_data.second_period.adventures
    }

    pub fn get_first_participations(&self) -> u64 {
        self.config_data.first_period.participations
    }

    pub fn get_second_participations(&self) -> u64 {
        self.config_data.second_period.participations
    }

    pub fn get_first_artifacts(&self) -> u64 {
        self.config_data.first_period.artifacts
    }

    pub fn get_second_artifacts(&self) -> u64 {
        self.config_data.second_period.artifacts
    }

    pub fn get_identified_artifacts(&self) -> u64 {
        self.config_data.second_period.identified_artifacts
    }

    pub fn get_start_date(&self) -> String {
        String::from(&self.config_data.first_period.start_date)
    }

    pub fn get_t1(&self) -> String {
        String::from(&self.config_data.first_period.date)
    }

    pub fn get_t2(&self) -> String {
        String::from(&self.config_data.second_period.date)
    }

    pub fn get_updated(&self) -> u64 {
        self.config_data.second_period.identified_artifacts
    }

    pub fn get_total_adventures(&self) -> u64 {
        self.get_first_adventures() + self.get_second_adventures()
    }

    pub fn get_total_adventurers(&self) -> u64 {
        self.get_first_adventurers() + self.get_second_adventurers()
    }
}
