mod adventurers; pub mod letter_data; pub mod model;
mod config;
use std::time::{Duration, Instant};
use adventurers::AdventurerGenerator;

use crate::config::ConfigReader;

fn main() {
    let mut config_data = ConfigReader::new("config.toml");
    config_data.get_config();
    let start = Instant::now();
    let adv = AdventurerGenerator::new(config_data.get_first_adventurers());
    adv.generate();
    let duration = start.elapsed();
    println!("Generated users in {:?}", duration)
}
