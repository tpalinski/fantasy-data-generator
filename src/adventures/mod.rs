mod status;
mod cost;
pub struct AdventureGenerator {
    rng: rand::rngs::ThreadRng,
    amount: u64,
    branches: u64,
    destinations: u64,
    start_date: String,
    t1_date: String,
}
