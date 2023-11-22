use std::f64::consts::PI;

use rand::Rng;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

// Returns a random double value in [0, 1).
#[inline]
pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

// Returns a random double value in [min, max).
#[inline]
pub fn random_double_ranged(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}
