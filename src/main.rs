mod adventurers;
pub mod letter_data;
pub mod model;
use std::time::{Duration, Instant};
use adventurers::AdventurerGenerator;

fn main() {
    let start = Instant::now();
    let adv = AdventurerGenerator::new(200000);
    adv.generate();
    let duration = start.elapsed();
    println!("Generated users in {:?}", duration)
}
