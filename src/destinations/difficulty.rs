use rand::Rng;

pub struct Generator {
    rng: rand::rngs::ThreadRng
}

impl Generator {
    pub fn new() -> Self{
        Generator{rng: rand::thread_rng()}
    }

    fn generate_single(&mut self) -> u32 {
        self.rng.gen_range(1..21)
    }

    pub fn generate_multiple(&mut self, amount: u64) -> Vec<u32> {
        let mut generated: Vec<u32> = vec![];
        for _ in 0..amount {
            let level = self.generate_single();
            generated.push(level);
        }
        generated
    }
}
