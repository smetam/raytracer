use std::ops::Range;
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Vec3, Point3};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Material,
    pub time: f64,
    pub outside: bool,
}

impl HitRecord {
    pub fn new<H: Hittable>(ray: &Ray, hittable: &H, time: f64) -> Self {
        let hit_point = ray.position_at(time);
        let normal = hittable.normal(&hit_point);
        let outside = ray.direction.dot(&normal) < 0.;
        Self {
            point: hit_point,
            normal: if outside { normal } else { -normal },
            material: hittable.material(),
            time,
            outside,
        }
    }


}

pub trait Hittable {

    /// This should always return a vector that has a unit length.
    fn normal(&self, hit_point: &Point3) -> Vec3;

    fn material(&self) -> Material;

    fn hit(&self, ray: &Ray, time_range: &Range<f64>) -> Option<HitRecord>;
}