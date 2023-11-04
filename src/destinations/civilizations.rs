use std::fs::File;

use rand::Rng;

use crate::model::civilization::Civilization;

pub struct Generator {
    rng: rand::rngs::ThreadRng,
    data: Vec<Civilization>,
}

impl Generator {
    pub fn new() -> Self {
        let data_path = "data/civilizations.csv";
        let file = File::open(data_path).expect("Error while opening civilizations.csv");
        let mut rdr = csv::Reader::from_reader(file);
        let civs = rdr.deserialize().map(|res| {
            let result: Civilization = match res {
                Ok(r) => r,
                Err(_) => {
                    panic!("Error while deserializing Civilizations");
                }
            };
            return result
        }).collect();
        Generator { rng: rand::thread_rng(), data: civs }
    }

    fn generate_single(&mut self) -> String {
        let index = self.rng.gen_range(0..self.data.len());
        self.data[index].get_name()
    }

    pub fn generate_multiple(&mut self, amount: u64) -> Vec<String> {
        let mut generated: Vec<String> = vec![];
        for _ in 0..amount {
            let name = self.generate_single();
            generated.push(name);
        }
        generated
    }
}
