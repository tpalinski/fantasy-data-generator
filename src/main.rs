mod adventurers; pub mod letter_data; pub mod model;
mod config;
mod destinations;
use std::time::{Duration, Instant};
use adventurers::AdventurerGenerator;

use crate::{config::ConfigReader, destinations::DestinationGenerator};

fn main() {
    let mut config_data = ConfigReader::new("config.toml");
    config_data.get_config();
    let start = Instant::now();
    let adv = AdventurerGenerator::new(config_data.get_first_adventurers());
    adv.generate();
    let duration = start.elapsed();
    println!("Generated users in {:?}", duration);
    let start = Instant::now();
    let mut dest = DestinationGenerator::new(config_data.get_first_destinations());
    dest.generate();
    let duration = start.elapsed();
    println!("Generated destinations in {:?}", duration);
}
