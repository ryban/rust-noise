// example.rs

extern crate noise;
extern crate image;
use noise::gen::NoiseGen;
use noise::gen::voronoi::Voronoi;
use noise::utils::bound;
use image::GenericImage;
use std::io::File;

fn main() {
    let mut ngen = Voronoi::new_rand();

    println!("Noise seed is {}", ngen.get_seed());
    
    let img_size = 512 as u32;
    let mut imbuf = image::ImageBuf::new(img_size, img_size);
    for x in range(0, img_size) {
        for y in range(0, img_size) {
            let n = ngen.get_value2d((x as f64)*0.075, (y as f64)*0.075);
            let nn = bound(n, 0.0, 1.0, -1.0, 1.0);
            let col = (nn * 255.0) as u8;
            let pixel = image::Luma(col);
            imbuf.put_pixel(x, y, pixel);
        }
    }

    let fout = File::create(&Path::new("voronoi.png")).unwrap();
    let _ = image::ImageLuma8(imbuf).save(fout, image::PNG);
    println!("voronoi.png saved")
}
