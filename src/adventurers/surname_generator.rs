use std::u8;
use rand;

use crate::letter_data::get_random_vowel;
use crate::letter_data::get_random_consonant;

pub struct Generator{
    rng: rand::rngs::ThreadRng
}

impl Generator {
    pub fn new() -> Self {
        Generator { rng: rand::thread_rng() }
    }

    fn generate_single(&mut self) -> String {
        return get_random_consonant(&mut self.rng).to_uppercase() + &get_random_consonant(&mut self.rng) + &get_random_vowel(&mut self.rng) + &get_random_consonant(&mut self.rng) + &get_random_vowel(&mut self.rng) + &get_random_consonant(&mut self.rng)
    }

    pub fn generate_multiple(&mut self, amount: u64) -> Vec<String>{
        let mut generated: Vec<String> = vec![];
        for _ in 0..amount {
            let surname = self.generate_single();
            generated.push(surname);
        }
        generated
    }
}
