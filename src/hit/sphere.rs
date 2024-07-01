use std::ops::Range;
use crate::hit::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Self {
        let radius = radius.max(0.);
        Self {center, radius, material}
    } 
}

impl Hittable for Sphere {
    fn normal(&self, hit_point: &Point3) -> Vec3 {
        &(hit_point - &self.center) / self.radius
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn hit(&self, ray: &Ray, time_range: &Range<f64>) -> Option<HitRecord> {
        let oc = &self.center - &ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None
        }
        let sqrt = discriminant.sqrt();

        let root = (h - sqrt) / a;
        if time_range.contains(&root) {
            return Some(HitRecord::new(ray, self, root))
        }

        let root = (h + sqrt) / a;
        if time_range.contains(&root) {
            return Some(HitRecord::new(ray, self, root))
        }

        None
    }
}