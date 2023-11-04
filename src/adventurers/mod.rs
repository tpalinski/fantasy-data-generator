use std::fs::File;

use crate::model::adventurer::Adventurer;


mod name_generator;
mod surname_generator;
mod level;

pub struct AdventurerGenerator {
    records_count: u64,
    generator: name_generator::Generator,
    surname_generator: surname_generator::Generator,
    level_generator: level::Generator
}

impl AdventurerGenerator {
    pub fn new(records: u64) -> Self {
        AdventurerGenerator{
            records_count: records,
            generator: name_generator::Generator::new(0),
            surname_generator: surname_generator::Generator::new(),
            level_generator: level::Generator::new(),
        }
    }

    pub fn generate(mut self) {
        let names = self.generator.generate_names(self.records_count);
        let surnames = self.surname_generator.generate_multiple(self.records_count);
        let levels = self.level_generator.generate_multiple(self.records_count);
        if names.len() != surnames.len() || names.len() != levels.len() {
            panic!("Error while generating user data")
        }
        let outfile = File::create("data/adventurers.csv").expect("Error: unable to open file");
        let mut wrt = csv::Writer::from_writer(outfile);
        for id in 0..names.len() {
            let saved = Adventurer::new(id as u64, String::from(&names[id]), String::from(&surnames[id]), levels[id]);
            let _ = wrt.serialize(saved);
        }
    }
}

