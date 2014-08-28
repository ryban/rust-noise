// ridgedmulti.rs

use gen::NoiseGen;
use gen::simplex::Simplex;
use utils::{clamp, bound};
use std::num;

pub struct RidgedMulti {
    simp: Simplex,
    octaves: int,
    gain: f64,
    lacunarity: f64,
    offset: f64,
    frequencies: Vec<f64>,
    max_val: f64
}

impl RidgedMulti {
    pub fn new_rand(octaves: int, gain: f64, lac: f64, offset: f64, h: f64) -> RidgedMulti {
        let freqs = RidgedMulti::calc_freqs(octaves, lac, h);
        RidgedMulti {  
            simp: Simplex::new_rand(),
            octaves: octaves,
            gain: gain,
            lacunarity: lac,
            offset: offset,
            frequencies: freqs.clone(),
            max_val: RidgedMulti::calc_max(octaves, gain, offset, freqs)
            }
    }

    pub fn from_seed(seed: u32, octaves: int, gain: f64, lac: f64, offset: f64, h: f64) -> RidgedMulti {
        let freqs = RidgedMulti::calc_freqs(octaves, lac, h);
        RidgedMulti {  
            simp: Simplex::from_seed(seed),
            octaves: octaves,
            gain: gain,
            lacunarity: lac,
            offset: offset,
            frequencies: freqs.clone(),
            max_val: RidgedMulti::calc_max(octaves, gain, offset, freqs)
            }
    }

    pub fn get_seed(&mut self) -> u32 {
        self.simp.get_seed()
    }

    fn calc_freqs(octaves: int, lacunarity: f64, h: f64) -> Vec<f64> {
        let mut freqs: Vec<f64> = Vec::new();

        for i in range(0, octaves) {
            let f = lacunarity.powf((-i as f64) * h);
            freqs.push(f);
        }

        freqs
    }

    fn calc_max(octaves: int, gain: f64, offset: f64, freqs: Vec<f64>) -> f64 {
        let mut signal = RidgedMulti::ridge(0.0, offset);
        let mut weight: f64;
        let mut sum = signal;
        for i in range(0, octaves) {
            weight = signal * gain;
            weight = clamp(weight, 0.0, 1.0);
            signal = RidgedMulti::ridge(0.0, offset);
            signal *= weight;
            sum += signal * freqs[i as uint];
        }
        
        sum
    }

    fn ridge(n: f64, offset: f64) -> f64 {
        let n = offset - num::abs(n);
        n * n
    }
}

impl NoiseGen for RidgedMulti {
    fn get_value2d(&mut self, x: f64, y: f64) -> f64 {
        let mut xx = x;
        let mut yy = y;
        let mut signal = RidgedMulti::ridge(
                            self.simp.get_value2d(x, y),
                            self.offset);
        let mut sum = signal;
        let mut weight: f64;

        for i in range(0, self.octaves) {
            xx = xx * self.lacunarity;
            yy = yy * self.lacunarity;
            weight = signal * self.gain;
            weight = clamp(weight, 0.0, 1.0);
            signal = RidgedMulti::ridge(
                        self.simp.get_value2d(xx, yy),
                        self.offset);
            signal *= weight;
            sum += signal * self.frequencies[i as uint];
        }

        bound(sum, 0.0, 1.0, 0.0, self.max_val)
    }

    fn get_value3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut xx = x;
        let mut yy = y;
        let mut zz = z;
        let mut signal = RidgedMulti::ridge(
                            self.simp.get_value3d(x, y, z),
                            self.offset);
        let mut sum = signal;
        let mut weight: f64;

        for i in range(0, self.octaves) {
            xx = xx * self.lacunarity;
            yy = yy * self.lacunarity;
            zz = zz * self.lacunarity;
            weight = signal * self.gain;
            weight = clamp(weight, 0.0, 1.0);
            signal = RidgedMulti::ridge(
                        self.simp.get_value3d(xx, yy, zz),
                        self.offset);
            signal *= weight;
            sum += signal * self.frequencies[i as uint];
        }

        bound(sum, 0.0, 1.0, 0.0, self.max_val)
    }
}
