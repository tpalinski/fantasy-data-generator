use rand::Rng;
const MAX_COST: u64 = 2500;

pub fn get_random_cost(rng: &mut rand::rngs::ThreadRng) -> u64 {
    rng.gen_range(0..MAX_COST)
}
