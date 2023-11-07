use rand::Rng;

const TYPES_AMOUNT: usize = 5;
const ARTIFACT_TYPES: [&str; TYPES_AMOUNT] = ["weapon", "armor", "ring", "helmet", "wand"];

pub fn get_random_type(rng: &mut rand::rngs::ThreadRng) -> String {
    let rand = rng.gen_range(0..TYPES_AMOUNT);
    ARTIFACT_TYPES[rand].to_string()
}
