
// utils.rs

#![allow(dead_code)]

use std::num;
use gen::NoiseGen;

pub fn lerp(low: f64, high: f64, t: f64) -> f64 {
    ((high-low)*t)+low
}

pub fn ease_curve(t: f64) -> f64 {
    6.0 * num::pow(t, 5) - 15.0 * num::pow(t, 4) + 10.0 * num::pow(t, 3)
}

pub fn bilerp(x0y0: f64, x0y1: f64, x1y0: f64, x1y1: f64, x: f64, y: f64) -> f64 {
    let tx = ease_curve(x);
    let ty = ease_curve(y);

    let u = lerp(x0y0, x1y0, tx);
    let v = lerp(x0y1, x1y1, tx);

    lerp(u, v, ty)
}

pub fn trilerp(v000: f64, v100: f64, v010: f64, v110: f64,
               v001: f64, v101: f64, v011: f64, v111: f64,
               x: f64, y: f64, z: f64) -> f64 {
    
    let t0 = v000*(1.0-x)*(1.0-y)*(1.0-z);
    let t1 = v100*x*(1.0-y)*(1.0-z);
    let t2 = v010*(1.0-x)*y*(1.0-z);
    let t3 = v110*x*y*(1.0-z);
    let t4 = v001*(1.0-x)*(1.0-y)*z;
    let t5 = v101*x*(1.0-y)*z;
    let t6 = v011*(1.0-x)*y*z;
    let t7 = v111*x*y*z;
    
    t0+t1+t2+t3+t4+t5+t6+t7
}

pub fn blend_quintic(x: f64) -> f64 {
    x*x*x*(x*((x*6.0)-15.0)+10.0)
}

pub fn bound(n: f64, low: f64, high: f64, oldlow: f64, oldhigh: f64) -> f64 {
    let nn = (n-oldlow)/(oldhigh-oldlow);
    nn*(high-low)+low
}

pub fn clamp(n: f64, low: f64, high: f64) -> f64 {
    match n {
        m if m < low    => low,
        m if m > high   => high,
        m               => m
    }
}

pub fn select_2d<L: NoiseGen, H: NoiseGen>(
    control: f64, low_source: &mut L, high_source: &mut H,
    threshold: f64, falloff: f64,
    x: f64, y:f64) -> f64 {
    
    match control {
        n if n > (threshold + falloff) => {
            high_source.get_value2d(x, y)
        },
        n if n < (threshold - falloff) => {
            low_source.get_value2d(x, y)
        },
        n => {
            let upper = threshold + falloff;
            let lower = threshold - falloff;
            let nn = (n-lower)/(upper-lower);
            let blend = blend_quintic(nn);
            lerp(low_source.get_value2d(x, y),
                 high_source.get_value2d(x, y),
                 blend)
        }
    }
}

pub fn select_3d<L: NoiseGen, H: NoiseGen>(
    control: f64, low_source: &mut L, high_source: &mut H,
    threshold: f64, falloff: f64,
    x: f64, y: f64, z: f64) -> f64 {

    let n = control;
    match n {
        n if n > (threshold + falloff) => {
            high_source.get_value3d(x, y, z)
        },
        n if n < ( threshold - falloff) => {
            low_source.get_value3d(x, y, z)
        },
        _ => {
            let upper = threshold + falloff;
            let lower = threshold - falloff;
            let nn = (n-lower)/(upper-lower);
            let blend = blend_quintic(nn);

            lerp(low_source.get_value3d(x, y, z),
                 high_source.get_value3d(x, y, z),
                 blend)
        }
    }
}

pub fn step(n: f64, steps: &[f64]) -> f64 {
    let mut last_step = steps[0];
    for &s in steps.iter() {
        if n < s {
            return last_step;
        }
        last_step = s;
    }
    // If it gets this far, this will be the last value in steps
    last_step
}
