use std::fs::File;

use rand::Rng;

use crate::model::participation::Participation;

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

    fn generate_adventurer(&mut self) -> u64 {
        self.rng.gen_range(0..self.adventurers)
    }

    fn generate_adventure(&mut self) -> u64 {
        self.rng.gen_range(0..self.adventures)
    }

    pub fn generate(&mut self) {
        let file = File::create("data/participations.csv").expect("Error while opening participations.csv");
        let mut wrt = csv::Writer::from_writer(file);
        for _ in 0..self.generated {
            let adventure = self.generate_adventure();
            let adventurer = self.generate_adventurer();
            let record = Participation::new(adventurer, adventure);
            let _ = wrt.serialize(record);
        }
    }
}
