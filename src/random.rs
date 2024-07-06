use crate::vec3::Vec3;
use rand::{thread_rng, Rng};
use std::ops::Range;

pub fn random() -> f64 {
    thread_rng().gen_range(0.0..=1.0)
}

pub fn random_range(range: Range<f64>) -> f64 {
    thread_rng().gen_range(range)
}

/// Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
pub fn sample_square() -> Vec3 {
    Vec3::new(random() - 0.5, random() - 0.5, 0.)
}

pub fn clamp(number: f64, range: &Range<f64>) -> f64 {
    match number {
        x if x < range.start => range.start,
        x if x > range.end => range.end,
        x => x,
    }
}
