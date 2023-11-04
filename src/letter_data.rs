use rand::Rng;

const VOWELS: [&str; 6] = ["a", "e", "i", "o", "u", "y"];
const CONSONANTS: [&str; 20] = ["q", "w", "r", "t", "p", "s", "d", "f", "g", "h", "j", "k", "l", "z", "x", "c", "v", "b", "n", "m"];
const VOWEL_COUNT: usize = 6;
const CONSONANT_COUNT: usize = 20;

pub fn get_random_vowel(rng: &mut rand::rngs::ThreadRng) -> String {
    let index = rng.gen_range(0..VOWEL_COUNT);
    String::from(VOWELS[index])
}

pub fn get_random_consonant(rng: &mut rand::rngs::ThreadRng) -> String {
    let index = rng.gen_range(0..CONSONANT_COUNT);
    String::from(CONSONANTS[index])
}
