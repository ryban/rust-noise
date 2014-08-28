// src/gen/mod.rs

pub mod simplex;
pub mod fbm;
pub mod ridgedmulti;
// pub mod voronoi;
// pub mod select;

pub trait NoiseGen {
    fn get_value2d(&mut self, x: f64, y: f64) -> f64;
    fn get_value3d(&mut self, x: f64, y: f64, z: f64) -> f64;
}
