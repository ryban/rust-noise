// select.rs

extern crate noise;
extern crate image;
extern crate time;


use noise::gen::NoiseGen;
use noise::gen::fbm::FBM;
use noise::gen::ridgedmulti::RidgedMulti;
use noise::gen::billow::Billow;
use noise::utils::select_2d;
use image::GenericImage;
use std::io::File;
use time::precise_time_s;

fn main() {
    let zoom = 100.0;
    let mut control = FBM::new_rand(24, 0.5, 2.5, 4.0*zoom);
    let mut high = Billow::new_rand(24, 0.5, 2.5, zoom);
    let mut low = RidgedMulti::new_rand(24, 1.7, 1.9, 1.0, 0.75, zoom);
    let threshold = 0.5;
    let falloff = 0.1;
    
    let img_size = 512 as u32;
    let mut imbuf = image::ImageBuf::new(img_size, img_size);
    
    let start = precise_time_s();
    for x in range(0, img_size) {
        for y in range(0, img_size) {
            let xx = x as f64;
            let yy = y as f64;
            let control_n = control.get_value2d(xx, yy);
            let n = select_2d(
                            control_n, &mut low, &mut high,
                            threshold, falloff,
                            xx, yy
                            );
            let col = (n * 255.0) as u8;
            let pixel = image::Luma(col);
            imbuf.put_pixel(x, y, pixel);
        }
    }
    let end = precise_time_s();

    let fout = File::create(&Path::new("select.png")).unwrap();
    let _ = image::ImageLuma8(imbuf).save(fout, image::PNG);
    println!("select.png saved");
    println!("generated {} points in {} ms", img_size*img_size, (end-start)*1000.0);
}
