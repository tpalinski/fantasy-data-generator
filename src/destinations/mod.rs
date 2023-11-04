use std::fs::File;

use crate::model::destination::Destination;

mod name_data;
mod name_generator;
mod difficulty;
mod civilizations;
mod coordinates;

pub struct DestinationGenerator {
    destinations: u64,
    name_generator: name_generator::Generator,
    difficulty_generator: difficulty::Generator,
    civ_generator: civilizations::Generator,
    coordinate_generator: coordinates::Generator,
}

impl DestinationGenerator {
    pub fn new(destinations: u64) -> Self {
        DestinationGenerator { destinations, name_generator: name_generator::Generator::new(), difficulty_generator: difficulty::Generator::new(), civ_generator: civilizations::Generator::new(), coordinate_generator: coordinates::Generator::new() }
    }

    pub fn generate(&mut self) {
        let civs = self.civ_generator.generate_multiple(self.destinations);
        let names = self.name_generator.generate_multiple(self.destinations);
        let diffs = self.difficulty_generator.generate_multiple(self.destinations);
        let coords = self.coordinate_generator.generate_multiple(self.destinations);
        if civs.len() != names.len() || civs.len() != diffs.len() || civs.len() != coords.len() {
            panic!("Error while generating")
        }
        let file = File::create("data/destinations.csv").expect("Error opening destinations.csv");
        let mut wrt = csv::Writer::from_writer(file);
        for id in 0..civs.len() {
            let destination = Destination::new(id as u64, String::from(&names[id]), String::from(&coords[id]), diffs[id], String::from(&civs[id]));
            let _ = wrt.serialize(destination);
        }
    }
}


