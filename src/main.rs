mod adventurers; pub mod letter_data; pub mod model;
mod config;
mod destinations;
mod participations;
mod artifacts;
mod adventures;
use std::time::Instant;
use adventurers::AdventurerGenerator;

use crate::{config::ConfigReader, destinations::DestinationGenerator, participations::ParticipationsGenerator, artifacts::ArtifactGenerator, adventures::AdventureGenerator};

const FACILITIES: u64 = 10;

fn main() {
    println!("Fetching config data");
    let mut config_data = ConfigReader::new("config.toml");
    config_data.get_config();
    println!("Generating data for T1");
    //Generate adventurers
    let start = Instant::now();
    let mut adv = AdventurerGenerator::new(config_data.get_first_adventurers());
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
    // Generate artifacts
    let start = Instant::now();
    let mut art = ArtifactGenerator::new(config_data.get_first_artifacts(), config_data.get_first_adventures());
    art.generate();
    let duration = start.elapsed();
    println!("Generated artifacts in {:?}", duration);
    // Generate adventures
    let start = Instant::now();
    let mut adventures_generator = AdventureGenerator::new(config_data.get_first_adventures(), FACILITIES, config_data.get_first_destinations(), config_data.get_start_date(), config_data.get_t1());
    adventures_generator.generate();
    let duration = start.elapsed();
    println!("Generated adventures in {:?}", duration);

    // Generate and modify data for second period

    //Generate adventurers
    println!("Generating data for T2");
    let start = Instant::now();
    adv.append(config_data.get_second_adventurers());
    let duration = start.elapsed();
    println!("Generated users for T2 in {:?}", duration);
}
