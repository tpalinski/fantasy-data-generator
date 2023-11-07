use rand::Rng;

const STATUS_COUNT: usize = 4;
const STATUS: [&str; STATUS_COUNT] = ["planned", "in progress", "finished", "failed"];

pub fn get_random_status(rng: &mut rand::rngs::ThreadRng) -> String{
    let index = rng.gen_range(0..STATUS_COUNT);
    STATUS[index].to_string()
}
