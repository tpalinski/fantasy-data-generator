use super::name_data;

pub struct Generator {
    rng: rand::rngs::ThreadRng,
}

impl Generator {
    pub fn new() -> Self {
        Generator { rng: rand::thread_rng() }
    }

    fn generate_single(&mut self) -> String {
        name_data::get_place_name(&mut self.rng)
    }

    pub fn generate_multiple(&mut self, amount: u64) -> Vec<String> {
        let mut generated: Vec<String> = vec![];
        for _ in 0..amount {
            let name = self.generate_single();
            generated.push(name);
        }
        generated
    }

}
