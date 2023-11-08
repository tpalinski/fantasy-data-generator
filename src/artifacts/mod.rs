use std::{fs::File, ops::Range};

use rand::Rng;

use crate::{model::artifact::Artifact, data_extract};

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

    fn generate_single(&mut self, id: u64, adventure_range: &Range<u64>) -> Artifact {
        let kind = type_data::get_random_type(&mut self.rng);
        let status = status::ArtifactStatus::get_random_status();
        let adventure = self.rng.gen_range(adventure_range.to_owned());
        Artifact::new(id, adventure, status, kind)
    }

    fn generate_multiple(&mut self, starting_id: u64, amount: u64, adventure_range: Range<u64>) -> Vec<Artifact> {
        let mut res: Vec<Artifact> = vec![];
        for id in 0..amount {
            let gen = self.generate_single(id+starting_id, &adventure_range);
            res.push(gen);
        }
        res
    }

    pub fn generate(&mut self){
        let generated = self.generate_multiple(0, self.amount, 0..self.adventures);
        data_extract::DataExtract::write_data(generated, "data/artifacts.csv");
    }

    pub fn append(&mut self, amount: u64, mut updated: u64, total_adventures: u64) {
        let mut data: Vec<Artifact> = data_extract::DataExtract::get_data("data/artifacts.csv");
        for artifact in &mut data {
            if updated == 0 {break;}
            match artifact.identify() {
                Ok(_) => updated -= 1,
                Err(_) => {}
            }
        }
        let mut new_data = self.generate_multiple(self.amount, amount, self.adventures..total_adventures);
        data.append(&mut new_data);
        data_extract::DataExtract::write_data(data, "data/artifacts_t2.csv");
    }
}
