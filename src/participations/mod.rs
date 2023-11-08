use std::{fs::File, ops::Range};

use rand::Rng;

use crate::{model::participation::Participation, letter_data, data_extract};

pub struct ParticipationsGenerator {
    generated: u64,
    rng: rand::rngs::ThreadRng,
    adventurers: u64,
    adventures: u64,
}

impl ParticipationsGenerator {
    pub fn new(generated: u64, adventurers: u64, adventures: u64) -> Self {
        ParticipationsGenerator { generated, rng: rand::thread_rng(), adventurers, adventures }
    }

    fn generate_adventurer(&mut self, adventurer_range: &Range<u64>) -> u64 {
        self.rng.gen_range(adventurer_range.to_owned())
    }

    fn generate_adventure(&mut self, adventure_range: &Range<u64>) -> u64 {
        self.rng.gen_range(adventure_range.to_owned())
    }

    fn generate_multiple(&mut self, amount: u64, adventure_range: Range<u64>, adventurer_range: Range<u64>) -> Vec<Participation> {
        let mut gen: Vec<Participation> = Vec::new();
        for _ in 0..amount {
            let adventure = self.generate_adventure(&adventure_range);
            let adventurer = self.generate_adventurer(&adventurer_range);
            let record = Participation::new(adventurer, adventure);
            gen.push(record);
        }
        gen
    }

    pub fn generate(&mut self) {
        let file = File::create("data/participations.csv").expect("Error while opening participations.csv");
        let mut wrt = csv::Writer::from_writer(file);
        let generated = self.generate_multiple(self.generated, 0..self.adventures, 0..self.adventurers);
        for record in generated {
            let _ = wrt.serialize(record);
        }
    }

    pub fn append(&mut self, amount: u64, total_adventures: u64, total_adventurers: u64) {
        let mut data: Vec<Participation> = data_extract::DataExtract::get_data("data/participations.csv");
        let mut new_data = self.generate_multiple(amount, self.adventures..total_adventures, 0..total_adventurers);
        data.append(&mut new_data);
        data_extract::DataExtract::write_data(data, "data/participations_t2.csv");

    }
}
