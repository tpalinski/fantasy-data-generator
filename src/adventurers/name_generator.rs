use std::u8;
use rand;

use crate::letter_data::get_random_vowel;
use crate::letter_data::get_random_consonant;


pub struct Generator {
    mode_mask: u8,
    current_mode: u8,
    rng: rand::rngs::ThreadRng
} 

impl Generator {
    pub fn new(mode_mask: u8) -> Self{
        Generator {
            mode_mask,
            current_mode: 0,
            rng: rand::thread_rng()
        }
    }

    fn generate_single(&mut self, mode: GenerationModes) -> String {
        mode.generate(&mut self.rng)
    }

    pub fn generate_names(&mut self, amount: u64) -> Vec<String>{
        let mut names: Vec<String> = vec![];
        for _ in 0..amount {
            let name = self.generate_single(GenerationModes::SYMMETRICAL);
            names.push(name);
        }
        names
    }
}

enum GenerationModes {
    SYMMETRICAL,
    CONSONANT,
    VOWEL,
    TAIL,
    HEAD
}

impl GenerationModes {
    pub fn from_number(number: u8) -> Self {
        match number {
            1 => Self::SYMMETRICAL,
            2 => Self::CONSONANT,
            4 => Self::VOWEL,
            8 => Self::TAIL,
            16 => Self::HEAD,
            _ => Self::SYMMETRICAL
        }
    }

    pub fn to_number(self) -> u8 {
        match self {
            Self::SYMMETRICAL => 1,
            Self::CONSONANT => 2,
            Self::VOWEL => 4,
            Self::TAIL => 8,
            Self::HEAD => 16
        }
    }

    pub fn generate(self, rng: &mut rand::rngs::ThreadRng) -> String {
        match self {
            Self::SYMMETRICAL => {
                get_random_consonant(rng) + &get_random_vowel(rng) + &get_random_consonant(rng) + &get_random_vowel(rng) + &get_random_consonant(rng)
            }
            _ => String::from("Not implemented")
        }
    }
}
