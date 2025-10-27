use std::time::{SystemTime, UNIX_EPOCH};

use rand::{Rng, SeedableRng, rngs::StdRng};

pub struct Random {
    rng: StdRng,
}

impl Default for Random {
    fn default() -> Self {
        Self::new()
    }
}

impl Random {
    pub fn new() -> Self {
        Self::seed(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }

    pub fn seed(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    pub fn u32(&mut self) -> u32 {
        self.rng.random()
    }

    pub fn f64(&mut self) -> f64 {
        self.rng.random()
    }

    pub fn range_u32(&mut self, low: u32, high: u32) -> u32 {
        self.rng.random_range(low..high)
    }

    pub fn range_f64(&mut self, low: f64, high: f64) -> f64 {
        self.rng.random_range(low..high)
    }

    pub fn chance(&mut self, (numerator, denominator): (f64, f64)) -> bool {
        if numerator == denominator {
            return true;
        }
        self.f64() < (numerator / denominator)
    }
}
