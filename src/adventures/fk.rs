use rand::Rng;

pub fn get_random_branch(rng: &mut rand::rngs::ThreadRng, facilities: u64) -> u64 {
    rng.gen_range(0..facilities)
} 


pub fn get_random_destination(rng: &mut rand::rngs::ThreadRng, destination: u64) -> u64 {
    rng.gen_range(0..destination)
} 
