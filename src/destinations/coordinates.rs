use rand::Rng;

const COORDINATE_RANGE: std::ops::Range<i32> = -16384..16384;

pub struct Generator {
    rng: rand::rngs::ThreadRng,
}

impl Generator {
    pub fn new() -> Self{
        Generator{rng: rand::thread_rng()}
    }

    fn generate_single(&mut self) -> String {
        let x = self.rng.gen_range(COORDINATE_RANGE);
        let y = self.rng.gen_range(COORDINATE_RANGE);
        x.to_string() + ", " + &y.to_string() 
    }

    pub fn generate_multiple(&mut self, amount: u64) -> Vec<String> {
        let mut generated: Vec<String> = vec![];
        for _ in 0..amount {
            let coords = self.generate_single();
            generated.push(coords);
        }
        generated
    }
}
