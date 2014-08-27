
// utils.rs

#![allow(dead_code)]

use std::num;

pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    ((a-b)*t)+a
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

/*
pub fn test() -> () {
    assert_eq!(lerp(0.0, 10.0, 0.5), 5.0);

    // Totally not legit
    assert_eq!(bilerp(0.0, 0.0, 0.0, 0.0, 0.0, 0.0), 0.0);

    // Totally not legit
    assert_eq!(trilerp( 0.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 0.0, 0.0,
                        0.0, 0.0, 0.0), 0.0);

    assert_eq!(bound(2.0, 0.0, 1.0, 0.0, 2.0), 1.0);

    assert_eq!(clamp(100.0, -1.0, 1.0), 1.0);
    assert_eq!(clamp(-100.0, -1.0, 1.0), -1.0);
    assert_eq!(clamp(0.0, -1.0, 1.0), 0.0);

    assert_eq!(ease_curve(0.0), 0.0);
}
*/
