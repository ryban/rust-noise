
use gen::NoiseGen;
use gen::simplex::Simplex;

pub struct Voronoi {
    simp: Simplex,
    zoom: f64
}

impl Voronoi {
    pub fn new_rand(zoom: f64) -> Voronoi {
        Voronoi { simp: Simplex::new_rand(1.0), zoom: 1.0/zoom }
    }

    pub fn from_seed(seed:u32, zoom: f64) -> Voronoi {
        Voronoi { simp: Simplex::from_seed(seed, 1.0), zoom: 1.0/zoom }
    }

    pub fn get_seed(&mut self) -> u32 {
        self.simp.get_seed()
    }
}

impl NoiseGen for Voronoi {
    fn get_value2d(&mut self, x: f64, y: f64) -> f64 {
        let xx = x * self.zoom;
        let yy = y * self.zoom;
        let xi = xx.floor() as int;
        let yi = yy.floor() as int;

        let mut min_dist = 2147483647.0; // 2^31
        // candidates for our x and y values
        let mut x_can = 0.0;
        let mut y_can = 0.0;

        for cur_y in range(yi-2, yi+2+1) {
            for cur_x in range(xi-2, xi+2+1) {
                let n = self.simp.get_value2d(cur_x as f64, cur_y as f64);
                let x_pos = cur_x as f64 + n;
                let y_pos = cur_y as f64 + n;
                let x_dist = x_pos - xx;
                let y_dist = y_pos - yy;
                let dist = (x_dist*x_dist) + (y_dist*y_dist);

                if dist < min_dist {
                    min_dist = dist;
                    x_can = x_pos;
                    y_can = y_pos;
                }
            }
        }

        self.simp.get_value2d(x_can.floor(), y_can.floor())
    }

    fn get_value3d(&mut self, x: f64, y: f64, z: f64) -> f64 {
        let xx = x * self.zoom;
        let yy = y * self.zoom;
        let zz = z * self.zoom;
        let xi = xx.floor() as int;
        let yi = yy.floor() as int;
        let zi = zz.floor() as int;

        let mut min_dist = 2147483647.0; // 2^31
        // candidates for our x and y values
        let mut x_can = 0.0;
        let mut y_can = 0.0;
        let mut z_can = 0.0;

        for cur_z in range(zi-2, zi+2+1){
            for cur_y in range(yi-2, yi+2+1) {
                for cur_x in range(xi-2, xi+2+1) {
                    let n = self.simp.get_value3d(  cur_x as f64,
                                                    cur_y as f64,
                                                    cur_z as f64);
                    let x_pos = cur_x as f64 + n;
                    let y_pos = cur_y as f64 + n;
                    let z_pos = cur_z as f64 + n;
                    let x_dist = x_pos - xx;
                    let y_dist = y_pos - yy;
                    let z_dist = z_pos - zz;
                    let dist = (x_dist*x_dist) + (y_dist*y_dist) + (z_dist*z_dist);

                    if dist < min_dist {
                        min_dist = dist;
                        x_can = x_pos;
                        y_can = y_pos;
                        z_can = z_pos;
                    }
                }
            }
        }

        self.simp.get_value3d(x_can.floor(), y_can.floor(), z_can.floor())
    }
}

