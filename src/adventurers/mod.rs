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
        let outfile = File::create("data/adventurers.csv").expect("Error: unable to open file");
        let mut wrt = csv::Writer::from_writer(outfile);
        for record in self.generate_multiple(0, self.records_count) {
            let _ = wrt.serialize(record);
        }
    }

    pub fn append(&mut self, amount: u64) {
        let file = File::open("data/adventurers.csv").expect("Error while opening adventurers.csv");
        let mut rdr = csv::Reader::from_reader(file);
        let mut old_data: Vec<Adventurer> = rdr.deserialize().map(|r| {
            let res: Adventurer = match r {
               Ok(record) => record,
                Err(_) => {
                    panic!("Error while deserializing adventurers");
                }
            };
            return res;
        }).collect();
        let mut new_data = self.generate_multiple(self.records_count.try_into().unwrap(), amount);
        old_data.append(&mut new_data);
        let outfile = File::create("data/adventurers_t2.csv").expect("Error opening adventurers_t2.csv");
        let mut wrt = csv::Writer::from_writer(outfile);
        for record in old_data {
            let _ = wrt.serialize(record);
        }
    }
}

