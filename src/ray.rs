use crate::hit::hittable::HitRecord;
use crate::materials::Scatter;
use crate::vec3::{Point3, Vec3};

pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {

    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn position_at(&self, time: f64) -> Point3 {
        let distance = &self.direction * time;
        &self.origin + distance
    }
}