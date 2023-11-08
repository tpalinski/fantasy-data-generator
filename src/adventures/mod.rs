use std::fs::File;

use crate::{model::adventure::Adventure, data_extract};

mod status;
mod cost;
mod dates;
mod fk;
pub struct AdventureGenerator {
    rng: rand::rngs::ThreadRng,
    amount: u64,
    branches: u64,
    destinations: u64,
    start_date: String,
    t1_date: String,
}

impl AdventureGenerator {
    pub fn new(amount: u64, branches: u64, destinations: u64, start_date: String, t1_date: String) -> Self{
        AdventureGenerator { rng: rand::thread_rng(), amount, branches, destinations, start_date, t1_date }
    }

    fn generate_single(&mut self, id: u64, start_date: &str, end_date: &str) -> Adventure {
        let facility = fk::get_random_branch(&mut self.rng, self.branches);
        let destination = fk::get_random_destination(&mut self.rng, self.destinations);
        let date = dates::get_random_date(String::from(start_date), String::from(end_date));
        let deadline = dates::get_random_deadline(date, &mut self.rng);
        let date = dates::date_to_string(date);
        let deadline = dates::date_to_string(deadline);
        let status = status::get_random_status(&mut self.rng);
        let cost = cost::get_random_cost(&mut self.rng);
        Adventure::new(id, facility, destination, date, deadline, status, cost)
    }

    fn generate_multiple(&mut self, amount: u64, id_offset: u64, start_date: &str, end_date: &str) -> Vec<Adventure> {
        let mut gen: Vec<Adventure> = Vec::new();
        for id in 0..amount {
            let a =  self.generate_single(id+id_offset, start_date, end_date);
            gen.push(a);
        }
        gen
    }

    pub fn generate(&mut self) -> () {
        let data = self.generate_multiple(self.amount, 0, &self.start_date.clone(), &self.t1_date.clone());
        data_extract::DataExtract::write_data(data, "data/adventures.csv");
    }

    pub fn append(&mut self, amount: u64, previous_adventures: u64, end_date: String) -> (){
        let mut data: Vec<Adventure> = data_extract::DataExtract::get_data("data/adventures.csv");
        let mut new_data = self.generate_multiple(amount, previous_adventures, &self.t1_date.clone(), &end_date);
        data.append(&mut new_data);
        data_extract::DataExtract::write_data(data, "data/adventures_t2.csv");
    }
}
