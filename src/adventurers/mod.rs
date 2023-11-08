use std::fs::File;

use crate::{model::adventurer::Adventurer, data_extract};


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

    fn generate_multiple(&mut self, starting_id: u64, amount: u64) -> Vec<Adventurer> {
        let mut res: Vec<Adventurer> = vec![];
        let names = self.generator.generate_names(amount);
        let surnames = self.surname_generator.generate_multiple(amount);
        let levels = self.level_generator.generate_multiple(amount);
        if names.len() != surnames.len() || names.len() != levels.len() {
            panic!("Error while generating user data")
        }
        for id in 0..names.len() {
            let saved = Adventurer::new(id as u64+starting_id, String::from(&names[id]), String::from(&surnames[id]), levels[id]);
            res.push(saved);
        }
        res
    }

    pub fn generate(&mut self) {
        let data = self.generate_multiple(0, self.records_count); 
        data_extract::DataExtract::write_data(data, "data/adventurers.csv");
    }

    pub fn append(&mut self, amount: u64) {
        let mut old_data: Vec<Adventurer> = data_extract::DataExtract::get_data("data/adventurers.csv");
        let mut new_data = self.generate_multiple(self.records_count.try_into().unwrap(), amount);
        old_data.append(&mut new_data);
        data_extract::DataExtract::write_data(old_data, "data/adventurers_t2.csv");
    }
}

