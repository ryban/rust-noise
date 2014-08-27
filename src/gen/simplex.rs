/*
 Simplex module
 Reference implementation:
    http://webstaff.itn.liu.se/~stegu/simplexnoise/SimplexNoise.java
*/

use std::num::Float;
use std::rand;
use std::rand::{Rng, SeedableRng};
use lcgrng::LCG;
use gen::NoiseGen;

pub struct Simplex {
    seed: u32,
    // 512 to remove the need for bounding array indicies
    perm: [uint, ..256]
}

impl NoiseGen for Simplex {
    fn get_value2d(&mut self, x: f64, y: f64) -> f64 {
        // sqrt(3) = 1.7320508075688772935274463415059
        static sqrt3: f64 = 1.7320508075688772935274463415059;
        static F2: f64 = 0.5*(sqrt3-1.0);
        static G2: f64 = (3.0-sqrt3)/6.0;

        static grad:[[int, ..2], ..12] = [[1,1],[-1,1],[1,-1],[-1,-1],
                                          [1,0],[-1,0],[1,0],[-1,0],
                                          [0,1],[0,-1],[0,1],[0,-1]];

        let mut n0: f64;
        let mut n1: f64;
        let mut n2: f64;

        let s: f64 = (x+y)*F2;
        let i = (x+s).floor() as int;
        let j = (x+s).floor() as int;

        let t: f64 = ((i+j) as f64)*G2;
        let x0: f64 = x-((i as f64)-t);
        let y0: f64 = y-((j as f64)-t);

        let mut i1: uint;
        let mut j1: uint;

        if x0 > y0 {
            i1 = 1;
            j1 = 0;
        } else {
            i1 = 0;
            j1 = 1;
        }

        let x1: f64 = x0-((i1 as f64)+G2);
        let y1: f64 = y0-((j1 as f64)+G2);
        let x2: f64 = x0-1.0+2.0*G2;
        let y2: f64 = y0-1.0+2.0*G2;

        let ii = (i & 0xff) as uint;
        let jj = (j & 0xff) as uint;

        let gi0 = (self.perm[(ii+self.perm[(jj)%256])%256] % 12) as uint;
        let gi1 = (self.perm[(ii+i1+self.perm[(jj+j1)%256])%256] % 12) as uint;
        let gi2 = (self.perm[(ii+1u+self.perm[(jj+1u)%256])%256] % 12) as uint;

        let t0: f64 = 0.5-(x0*x0)-(y0*y0);
        if t0 < 0.0 {
            n0 = 0.0;
        } else {
            n0 = t0*t0*t0*t0*Simplex::dot2(&grad[gi0], x0, y0);
        }

        let t1: f64 = 0.5-(x1*x1)-(y1*y1);
        if t1 < 0.0 {
            n1 = 0.0;
        } else {
            n1 = t1*t1*t1*t1*Simplex::dot2(&grad[gi1], x1, y1);
        }

        let t2: f64 = 0.5-(x2*x2)-(y2*y2);
        if t2 < 0.0 {
            n2 = 0.0;
        } else {
            n2 = t2*t2*t2*t2*Simplex::dot2(&grad[gi2], x2, y2);
        }

        70.0*(n0+n1+n2)
    }

    fn get_value3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        static F3: f64 = 1.0/3.0;
        static G3: f64 = 1.0/6.0;

        static grad: [[int, ..3], ..12] = [[1,1,0],[-1,1,0],[1,-1,0],[-1,-1,0],
                                           [1,0,1],[-1,0,1],[1,0,-1],[-1,0,-1],
                                           [0,1,1],[0,-1,1],[0,1,-1],[0,-1,-1]];

        let mut n0: f64;
        let mut n1: f64;
        let mut n2: f64;
        let mut n3: f64;

        let s: f64 = (x+y+z)*F3;
        let i = (x+s).floor() as int;
        let j = (y+s).floor() as int;
        let k = (z+s).floor() as int;

        let t: f64 = ((i+j+k) as f64)*G3;
        let x0: f64 = x-((i as f64)-t);
        let y0: f64 = y-((j as f64)-t);
        let z0: f64 = z-((k as f64)-t);

        let mut i1: uint;
        let mut j1: uint;
        let mut k1: uint;
        let mut i2: uint;
        let mut j2: uint;
        let mut k2: uint;

        if x0 >= y0 {
            if y0 >= z0 {
                i1 = 1; j1 = 0; k1 = 0; i2 = 1; j2 = 1; k2 = 0;
            }else if x0 >= z0 {
                i1 = 1; j1 = 0; k1 = 0; i2 = 1; j2 = 0; k2 = 1;
            }else {
                i1 = 0; j1 = 0; k1 = 1; i2 = 1; j2 = 0; k2 = 1;
            }
        }else { // x0 < y0
            if y0 < z0 {
                i1 = 0; j1 = 0; k1 = 1; i2 = 0; j2 = 1; k2 = 1;
            }else if x0 < z0 {
                i1 = 0; j1 = 1; k1 = 0; i2 = 0; j2 = 1; k2 = 1;
            }else {
                i1 = 0; j1 = 1; k1 = 0; i2 = 1; j2 = 1; k2 = 0;
            }
        }

        let x1: f64 = x0 - (i1 as f64) + G3;
        let y1: f64 = y0 - (j1 as f64) + G3;
        let z1: f64 = z0 - (k1 as f64) + G3;
        let x2: f64 = x0 - (i2 as f64) + 2.0*G3;
        let y2: f64 = y0 - (j2 as f64) + 2.0*G3;
        let z2: f64 = z0 - (k2 as f64) + 2.0*G3;
        let x3: f64 = x0 - 1.0 + 3.0*G3;
        let y3: f64 = y0 - 1.0 + 3.0*G3;
        let z3: f64 = z0 - 1.0 + 3.0*G3;

        let ii = (i & 0xff) as uint;
        let jj = (j & 0xff) as uint;
        let kk = (k & 0xff) as uint;

        let gi0 = (self.perm[ (ii+   self.perm[ (jj+   self.perm[  kk         ])%256 ])%256 ] % 12) as uint;
        let gi1 = (self.perm[ (ii+i1+self.perm[ (jj+j1+self.perm[ (kk+k1)%256 ])%256 ])%256 ] % 12) as uint;
        let gi2 = (self.perm[ (ii+i2+self.perm[ (jj+j2+self.perm[ (kk+k2)%256 ])%256 ])%256 ] % 12) as uint;
        let gi3 = (self.perm[ (ii+1u+self.perm[ (jj+1u+self.perm[ (kk+1u)%256 ])%256 ])%256 ] % 12) as uint;
    
        let t0 = 0.6 - (x0*x0) - (y0*y0) - (z0*z0);
        if t0 < 0.0 {
            n0 = 0.0;
        } else {
            n0 = t0*t0*t0*t0*Simplex::dot3(&grad[gi0], x0, y0, z0);
        }

        let t1 = 0.6 - (x1*x1) - (y1*y1) - (z1*z1);
        if t1 < 0.0 {
            n1 = 0.0;
        } else {
            n1 = t1*t1*t1*t1*Simplex::dot3(&grad[gi1], x1, y1, z1);
        }

        let t2 = 0.6 - (x2*x2) - (y2*y2) - (z2*z2);
        if t2 < 0.0 {
            n2 = 0.0;
        } else {
            n2 = t2*t2*t2*t2*Simplex::dot3(&grad[gi2], x2, y2, z2);
        }

        let t3 = 0.6 - (x3*x3) - (y3*y3) - (z3*z3);
        if t3 < 0.0 {
            n3 = 0.0;
        } else {
            n3 = t3*t3*t3*t3*Simplex::dot3(&grad[gi3], x3, y3, z3);
        }

        32.0*(n0+n1+n2+n3)
    }
}

impl Simplex {
    pub fn new_rand() -> Simplex {
        let mut rng = rand::task_rng(); // for getting a random seed
        let mut simp = Simplex { seed: rng.gen(), perm: [0, ..256] };
        simp.init_perm();
        simp
    }

    pub fn from_seed(seed: u32) -> Simplex {
        let mut simp = Simplex { seed: seed, perm: [0, ..256] };
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
            self.perm[i] = i as uint;
        }
        rng.shuffle(self.perm);
    }

    fn dot3(g: &[int, ..3], x: f64, y: f64, z: f64) -> f64 {
        ((g[0] as f64)*x )+ ((g[1] as f64)*y) + ((g[2] as f64)*z)
    }

    fn dot2(g: &[int, ..2], x: f64, y: f64) -> f64 {
        ((g[0] as f64)*x) + ((g[1] as f64)*y)
    }
}
