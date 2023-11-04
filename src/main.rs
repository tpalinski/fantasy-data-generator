mod adventurers; pub mod letter_data; pub mod model;
mod config;
mod destinations;
mod participations;
use std::time::Instant;
use adventurers::AdventurerGenerator;

use crate::{config::ConfigReader, destinations::DestinationGenerator, participations::ParticipationsGenerator};

fn main() {
    let mut config_data = ConfigReader::new("config.toml");
    config_data.get_config();
    //Generate adventurers
    let start = Instant::now();
    let adv = AdventurerGenerator::new(config_data.get_first_adventurers());
    adv.generate();
    let duration = start.elapsed();
    println!("Generated users in {:?}", duration);
    // Generate adventure destinations
    let start = Instant::now();
    let mut dest = DestinationGenerator::new(config_data.get_first_destinations());
    dest.generate();
    let duration = start.elapsed();
    println!("Generated destinations in {:?}", duration);
    // Generate participations in adventures
    let start = Instant::now();
    let mut participations = ParticipationsGenerator::new(config_data.get_first_participations(), config_data.get_first_adventurers(), config_data.get_first_adventures());
    participations.generate();
    let duration = start.elapsed();
    println!("Generated participations in {:?}", duration);
}
