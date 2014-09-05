
use gen::NoiseGen;
use gen::simplex::Simplex;
use utils::bound;

pub struct FBM {
    simp: Simplex,
    octaves: int,
    persitence: f64,
    gain: f64,
    max_val: f64
}

impl FBM {
    pub fn new_rand(octaves: int, persitence: f64, gain: f64, zoom: f64) -> FBM {
        FBM {   simp: Simplex::new_rand(zoom),
                octaves: octaves,
                persitence: persitence,
                gain: gain,
                max_val: FBM::calc_max(octaves, persitence)
            }
    }

    pub fn from_seed(seed:u32, octaves: int, persitence: f64, gain: f64, zoom: f64) -> FBM {
        FBM {   simp: Simplex::from_seed(seed, zoom),
                octaves: octaves,
                persitence: persitence,
                gain: gain,
                max_val: FBM::calc_max(octaves, persitence)
            }
    }

    pub fn get_seed(&mut self) -> u32 {
        self.simp.get_seed()
    }

    // static function for calculating the max/min values the noise can have
    // used to bound the noise to [-1,1]
    // If the octaves or persitence ever change, max_val must be recalculated
    fn calc_max(octaves: int, persitence: f64) -> f64 {
        let mut a = 1.0;
        let mut n = 0.0;
        for i in range(0, octaves) {
            n += a;
            a *= persitence;
        }

        n
    }
}

impl NoiseGen for FBM {
    fn get_value2d(&mut self, x: f64, y: f64) -> f64 {
        let mut n = 0.0;
        let mut f = 1.0;
        let mut amp = 1.0;

        for i in range(0, self.octaves) {
            n += amp * self.simp.get_value2d(x*f, y*f);
            f *= self.gain;
            amp *= self.persitence;
        }

        bound(n, 0.0, 1.0, -self.max_val, self.max_val)
    }

    fn get_value3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let mut n = 0.0;
        let mut f = 1.0;
        let mut amp = 1.0;

        for i in range(0, self.octaves) {
            n += amp * self.simp.get_value3d(x*f, y*f, z*f);
            f *= self.gain;
            amp *= self.persitence;
        }

        bound(n, 0.0, 1.0, -self.max_val, self.max_val)
    }
}

