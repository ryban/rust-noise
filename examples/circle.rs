// step.rs

extern crate noise;
extern crate image;
use noise::gen::NoiseGen;
use noise::gen::ridgedmulti::RidgedMulti;
use noise::utils::circle;
use image::GenericImage;
use std::io::File;

fn main() {
    let mut inside = RidgedMulti::new_rand(24, 1.7, 1.9, 1.0, 0.75, 100.0);
    let mut outside: f64 = 0.0;
    let falloff = 100.0;

    let img_size = 512 as u32;
    let mut imbuf = image::ImageBuf::new(img_size, img_size);

    let radius: f64 = (img_size as f64) / 3.0;
    let center: f64 = (img_size as f64) / 2.0; 
    for x in range(0, img_size) {
        for y in range(0, img_size) {
            let xx = x as f64;
            let yy = y as f64;
            let n = circle(radius, center, center, falloff,
                            &mut inside, &mut outside,
                            xx, yy);
            let col = (n * 255.0) as u8;
            let pixel = image::Luma(col);
            imbuf.put_pixel(x, y, pixel);
        }
    }

    let fout = File::create(&Path::new("circle.png")).unwrap();
    let _ = image::ImageLuma8(imbuf).save(fout, image::PNG);
    println!("circle.png saved");
}