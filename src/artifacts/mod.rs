use std::{fs::File, ops::Range};

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
        let file = File::create("data/artifacts.csv").expect("Error while opening artifacts.csv");
        let mut wrt = csv::Writer::from_writer(file);
        let generated = self.generate_multiple(0, self.amount, 0..self.adventures);
        for record in generated {
            let _ = wrt.serialize(record);
        }
    }

    pub fn append(&mut self, amount: u64, mut updated: u64, total_adventures: u64) {
        let file = File::open("data/artifacts.csv").expect("Error while opening artifacts.csv");
        let mut rdr = csv::Reader::from_reader(file);
        let mut data: Vec<Artifact> = rdr.deserialize().map(|data| {
            let res: Artifact = match data {
                Ok(a) => a,
                Err(_) => {
                    panic!("Error while deserializing artifacts");
                }
            };
            return res;
        }).collect();
        for artifact in &mut data {
            if updated == 0 {break;}
            match artifact.identify() {
                Ok(_) => updated -= 1,
                Err(_) => {}
            }
        }
        let mut new_data = self.generate_multiple(self.amount, amount, self.adventures..total_adventures);
        data.append(&mut new_data);
        let outfile = File::create("data/artifacts_t2.csv").expect("Error while opening artifacts_t2.csv");
        let mut wrt = csv::Writer::from_writer(outfile);
        for record in data {
            let _ = wrt.serialize(record);
        }
    }
}
