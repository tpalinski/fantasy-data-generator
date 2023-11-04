use rand::Rng;

use crate::letter_data::{get_random_consonant, get_random_vowel};

const SUFFIXES_COUNT: usize = 10;
const SUFFIXES: [&str; SUFFIXES_COUNT] = ["'s Folly", "'s Tower of Inescapable Doom", "'s Sanctum", "'s Recluse", "'s House", "'s Tower of Escapable Doom", "'s Fishing Cottage", "'s Mountain Resort", " & Sons' Shop", "'s Magnificent Emporium"];

fn get_random_suffix(rng: &mut rand::rngs::ThreadRng) -> String {
    let index = rng.gen_range(0..SUFFIXES_COUNT);
    String::from(SUFFIXES[index])
}

fn get_random_name(rng: &mut rand::rngs::ThreadRng) -> String {
    get_random_consonant(rng).to_uppercase() + &get_random_vowel(rng) + &get_random_consonant(rng) + &get_random_vowel(rng) + &get_random_consonant(rng) + &get_random_vowel(rng) + &get_random_consonant(rng)
}

pub fn get_place_name(rng: &mut rand::rngs::ThreadRng) -> String {
    get_random_name(rng) + &get_random_suffix(rng)
}
