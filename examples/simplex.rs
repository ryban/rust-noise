// example.rs

extern crate noise;
extern crate image;
extern crate time;

use noise::gen::NoiseGen;
use noise::gen::simplex::Simplex;
use image::GenericImage;
use std::io::File;
use time::precise_time_s;

fn main() {
    let mut ngen = Simplex::new_rand(20.0);

    println!("Noise seed is {}", ngen.get_seed());
    
    let img_size = 512 as u32;
    let mut imbuf = image::ImageBuf::new(img_size, img_size);

    let start = precise_time_s();
    for x in range(0, img_size) {
        for y in range(0, img_size) {
            let n = ngen.get_value2d((x as f64), (y as f64));
            let col = (((n+1.0)/2.0) * 255.0) as u8;
            let pixel = image::Luma(col);
            imbuf.put_pixel(x, y, pixel);
        }
    }
    let end = precise_time_s();

    let fout = File::create(&Path::new("simplex.png")).unwrap();
    let _ = image::ImageLuma8(imbuf).save(fout, image::PNG);
    println!("simplex.png saved")
    println!("generated {} points in {} ms", img_size*img_size, (end-start)*1000.0);
}
