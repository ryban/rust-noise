// example.rs

// extern crate png;
extern crate noise;
extern crate png;
use noise::gen::NoiseGen;
use noise::gen::simplex::Simplex;
use png::{Image, RGB8, store_png};

fn main() {
    let mut ngen = Simplex::new_rand();
    // let mut ngen = Simplex::from_seed(90813409);

    println!("Noise seed is {}", ngen.get_seed());
    // println!("Here's some 2D noise");
    let img_size = 512i;

    let mut img = Image {
        width: img_size,
        height: img_size,
        pixels: RGB8(Vec::from_elem(img_size*img_size*3, 0u8));
    };
    for x in range(0, img_size) {
        for y in range(0, img_size) {
            n = ngen.get_value2d(x as f64, y as f64);
            let col = ((n+1.0/2.0) * 255) as u8;
            img.pixels[(y*img_size)+x+0] = col;
            img.pixels[(y*img_size)+x+1] = col;
            img.pixels[(y*img_size)+x+2] = col;
        }
    }
    let save_path = "simplex.png"
    store_png(img, save_path);

    println!("Here's some 3D noise");
    for i in range(-5i, 5i) {
        let ii = i as f64;
        println!("{:f}", ngen.get_value3d(ii, ii, ii));
    }
}
