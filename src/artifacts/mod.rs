use std::fs::File;

use rand::Rng;

use crate::model::artifact::Artifact;

mod status;
mod type_data;

pub struct ArtifactGenerator {
    amount: u64,
    adventures: u64,
    rng: rand::rngs::ThreadRng,
}
impl ArtifactGenerator {
    pub fn new(amount: u64, adventures: u64) -> Self {
        ArtifactGenerator { amount, adventures, rng: rand::thread_rng() }
    }

    fn generate_single(&mut self, id: u64) -> Artifact {
        let kind = type_data::get_random_type(&mut self.rng);
        let status = status::ArtifactStatus::get_random_status();
        let adventure = self.rng.gen_range(0..self.adventures);
        Artifact::new(id, adventure, status, kind)
    }

    pub fn generate(&mut self){
        let file = File::create("data/artifacts.csv").expect("Error while opening artifacts.csv");
        let mut wrt = csv::Writer::from_writer(file);
        for id in 0..self.amount {
            let gen = self.generate_single(id);
            let _ = wrt.serialize(gen);
        }
    }
}
