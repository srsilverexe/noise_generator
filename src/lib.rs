use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use rand::{self, Rng, SeedableRng};

pub struct Noise {
    pub values: Vec<f64>,
    pub len: u32,
    pub seed: <ChaCha8Rng as SeedableRng>::Seed,
}

impl Noise {
    pub fn generate_noise(&mut self) {
        thread_rng().fill(&mut self.seed);
        let mut rng = ChaCha8Rng::from_seed(self.seed);
        for _ in 0..self.len {
            self.values.insert(self.values.len(), rng.gen::<f64>());
        }
    }
}
