// step.rs

extern crate noise;
extern crate image;
use noise::gen::NoiseGen;
use noise::gen::fbm::FBM;
use noise::utils::step;
use image::GenericImage;
use std::io::File;

fn main() {
    let mut ngen = FBM::new_rand(24, 0.5, 2.5);
    let steps: &[f64] = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0];

    let img_size = 512 as u32;
    let mut imbuf = image::ImageBuf::new(img_size, img_size);
    for x in range(0, img_size) {
        for y in range(0, img_size) {
            let xx = (x as f64)*0.006;
            let yy = (y as f64)*0.006;
            let nn = ngen.get_value2d(xx, yy);
            let n = step(nn, steps);
            let col = (n * 255.0) as u8;
            let pixel = image::Luma(col);
            imbuf.put_pixel(x, y, pixel);
        }
    }

    let fout = File::create(&Path::new("step.png")).unwrap();
    let _ = image::ImageLuma8(imbuf).save(fout, image::PNG);
    println!("step.png saved");
}