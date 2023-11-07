use std::fs::File;

use crate::model::adventure::Adventure;

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

    fn generate_single(&mut self, id: u64) -> Adventure {
        let facility = fk::get_random_branch(&mut self.rng, self.branches);
        let destination = fk::get_random_destination(&mut self.rng, self.destinations);
        let date = dates::get_random_date(String::from(&self.start_date), String::from(&self.t1_date));
        let deadline = dates::get_random_deadline(date, &mut self.rng);
        let date = dates::date_to_string(date);
        let deadline = dates::date_to_string(deadline);
        let status = status::get_random_status(&mut self.rng);
        let cost = cost::get_random_cost(&mut self.rng);
        Adventure::new(id, facility, destination, date, deadline, status, cost)
    }

    pub fn generate(&mut self) -> () {
        let file = File::create("data/adventures.csv").expect("Error opening adventures.csv");
        let mut wrtr = csv::Writer::from_writer(file);
        for id in 0..self.amount {
            let adv = self.generate_single(id);
            let _ = wrtr.serialize(adv);
        }
    }
}
