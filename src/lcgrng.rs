// lcdrng.rs
// implimentation of 64 bit linear congruential generator

use std::rand::{Rng, SeedableRng};

pub struct LCG {
    seed: u32,
    prev_val: u32,
}

impl Rng for LCG {
    fn next_u32(&mut self) -> u32 {
        static A: u32 = 1664525;
        static C: u32 = 1013904223;
        self.prev_val = (A*self.prev_val)+C; // Normally a bit mask, but is it necessary?
        self.prev_val
    }
}

impl SeedableRng<u32> for LCG {
    fn reseed(&mut self, seed: u32) {
        self.seed = seed;
        self.prev_val = seed;
    }

    fn from_seed(seed: u32) -> LCG {
        let lcg = LCG {seed: seed, prev_val: seed};
        lcg
    }
}
