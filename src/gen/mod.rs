// src/gen/mod.rs

pub mod simplex;
pub mod fbm;
pub mod ridgedmulti;
pub mod billow;
pub mod voronoi;

pub trait NoiseGen {
    fn get_value2d(&mut self, x: f64, y: f64) -> f64;
    fn get_value3d(&mut self, x: f64, y: f64, z: f64) -> f64;
}


#[allow(unused_variable)]
impl NoiseGen for f64 {
    fn get_value2d(&mut self, x: f64, y: f64) -> f64 {
        *self
    }

    fn get_value3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        *self
    }
}
