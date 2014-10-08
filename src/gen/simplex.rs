/*
 Simplex module
 Reference implementation:
    http://webstaff.itn.liu.se/~stegu/simplexnoise/SimplexNoise.java
*/
use std::rand;
use std::rand::{Rng, SeedableRng};
use lcgrng::LCG;
use gen::NoiseGen;

pub struct Simplex {
    seed: u32,
    zoom: f64,
    // 512 to remove the need for bounding array indicies
    perm: [u8, ..256]
}

impl Simplex {
    pub fn new_rand(zoom: f64) -> Simplex {
        let mut rng = rand::task_rng(); // for getting a random seed
        let mut simp = Simplex { seed: rng.gen(), zoom: 1.0/zoom, perm: [0, ..256] };
        simp.init_perm();
        simp
    }

    pub fn from_seed(seed: u32, zoom: f64) -> Simplex {
        let mut simp = Simplex { seed: seed, zoom: 1.0/zoom, perm: [0, ..256] };
        simp.init_perm();
        simp
    }

    pub fn set_seed(&mut self, seed: u32) -> () {
        self.seed = seed;
        self.init_perm();
    }

    pub fn get_seed(&mut self) -> u32 {
        self.seed
    }

    fn init_perm(&mut self) -> () {
        let mut rng: LCG = SeedableRng::from_seed(self.seed);

        for i in range(0, 256) {
            self.perm[i] = i as u8;
        }
        rng.shuffle(self.perm);
    }

    fn extrapolate_2d(&mut self, xsb: int, ysb: int, dx: f64, dy: f64) -> f64 {
        static GRAD_2D: [i8, ..16] = [
             5,  2,    2,  5,
            -5,  2,   -2,  5,
             5, -2,    2, -5,
            -5, -2,   -2, -5,
        ];

        let xsb_idx = (xsb & 0xFF) as uint;
        let ysb_idx = (ysb & 0xFF) as uint;
        let idx = (self.perm[((self.perm[xsb_idx] as uint) + ysb_idx) & 0xFF] & 0x0E) as uint;
        
        ((GRAD_2D[idx] as f64) * dx) + ((GRAD_2D[idx+1u] as f64) * dy)
    }
}

impl NoiseGen for Simplex {
    // OpenSimplex implimentation: https://gist.github.com/KdotJPG/b1270127455a94ac5d19
    fn get_value2d(&mut self, x: f64, y: f64) -> f64 {
        static STRETCH_CONSTANT: f64 = -0.211324865405187;
        static SQUISH_CONSTANT: f64 = 0.366025403784439;
        static NORM_CONSTANT: f64 = 47.0;

        let x = x * self.zoom;
        let y = y * self.zoom;

        let stretch_offset = (x + y) * STRETCH_CONSTANT;
        let xs = x + stretch_offset;
        let ys = y + stretch_offset;

        // Floor to get grid coordinates of rhombus (stretched square) super cell origin
        let xsb = xs.floor() as int;
        let ysb = ys.floor() as int;

        // Skew out to get actual coordinates of rhombus origin
        let squish_offset = ((xsb + ysb) as f64) * SQUISH_CONSTANT;
        let xb = (xsb as f64) + squish_offset;
        let yb = (ysb as f64) + squish_offset;

        // Computer grid coordinates relative to rhombus origin
        let xins = xs - (xsb as f64);
        let yins = ys - (ysb as f64);

        // Sum those together to get a value that determines which region we're in
        let in_sum = xins + yins;

        // Position relative to origin point
        let dx0 = x - xb;
        let dy0 = y - yb;

        // Contribution (1,0)
        let dx1 = dx0 - 1.0 - SQUISH_CONSTANT;
        let dy1 = dy0 - 0.0 - SQUISH_CONSTANT;
        let attn1 = 2.0 - (dx1 * dx1) - (dy1 * dy1);

        let v1: f64 = if attn1 > 0.0 {
            let attn1_4 = attn1*attn1*attn1*attn1;
            attn1_4*self.extrapolate_2d(xsb+1, ysb+0, dx1, dy1)
        } else {
            0.0
        };

        // Contribution (0,1)
        let dx2 = dx0 - 0.0 - SQUISH_CONSTANT;
        let dy2 = dy0 - 1.0 - SQUISH_CONSTANT;
        let attn2 = 2.0 - (dx2 * dx2) - (dy2 * dy2);
        
        let v2: f64 = if attn2 > 0.0 {
            let attn2_4 = attn2*attn2*attn2*attn2;
            attn2_4*self.extrapolate_2d(xsb+0, ysb+1, dx2, dy2)
        } else {
            0.0
        };

        let (xsv_ext, ysv_ext, dx_ext, dy_ext) = if in_sum <= 1.0 { // We're inside the triangle (2-Simplex) at (0,0)
            let zins = 1.0 - in_sum;
            if zins > xins || zins > yins { // (0,0) is one of the closest two triangular vertices
                if xins > yins {
                    (xsb+1, ysb-1, dx0-1.0, dy0+1.0)
                } else {
                    (xsb-1, ysb+1, dx0+1.0, dy0-1.0)
                }
            }else { // (1,0) and (0,1) are the closest two vertices.
                (xsb+1, ysb+1, dx0-1.0-(2.0*SQUISH_CONSTANT), dy0-1.0-(2.0*SQUISH_CONSTANT))
            }
        }else { // We're inside the triangle (2-Simplex) at (1,1)
            let zins = 2.0 - in_sum;
            if zins < xins || zins < yins { // (0,0) is one of the closest two triangular vertices
                if xins > yins {
                    (xsb+2, ysb+0, dx0-2.0-(2.0*SQUISH_CONSTANT), dy0+0.0-(2.0*SQUISH_CONSTANT))
                }else {
                    (xsb+0, ysb+2, dx0-0.0-(2.0*SQUISH_CONSTANT), dy0-2.0-(2.0*SQUISH_CONSTANT))
                }
            }else { // (1,0) and (0,1) are the closest two vertices.
                (xsb, ysb, dx0, dy0)
            }
        };
        // In the original implementation this was done in the above,
        // but I think this is nicer
        let (xsb, ysb, dx0, dy0) = if in_sum <= 1.0 {
            (xsb, ysb, dx0, dy0)
        }else {
            (xsb + 1,
             ysb + 1,
             dx0 - 1.0 - (2.0 * SQUISH_CONSTANT),
             dy0 - 1.0 - (2.0 * SQUISH_CONSTANT))
        };

        // Contribution (0,0) or (1,1)
        let attn0 = 2.0 - (dx0 * dx0) - (dy0 * dy0);
        let v0: f64 = if attn0 > 0.0 {
            let attn0_4 = attn0*attn0*attn0*attn0;
            attn0_4*self.extrapolate_2d(xsb, ysb, dx0, dy0)
        } else {
            0.0
        };

        // Extra vertex
        let attn_ext = 2.0 - (dx_ext * dx_ext) - (dy_ext * dy_ext);
        let v_ext: f64 = if attn_ext > 0.0 {
            let attn_ext_4 = attn_ext*attn_ext*attn_ext*attn_ext;
            attn_ext_4*self.extrapolate_2d(xsv_ext, ysv_ext, dx_ext, dy_ext)
        } else {
            0.0
        };

        (v0 + v1 + v2 + v_ext) / NORM_CONSTANT
    }

    fn get_value3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        // TODO: All of this
        x + y + z
    }
}
