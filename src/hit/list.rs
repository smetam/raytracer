use std::cmp::Ordering;
use std::ops::Range;
use crate::hit::hittable::{HitRecord, Hittable};
use crate::materials::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn add<H: Hittable + 'static>(&mut self, object: H) {
        self.list.push(Box::new(object))
    }

    pub fn clear(&mut self) {
        self.list.clear()
    }
}

impl Hittable for HittableList {
    fn normal(&self, hit_point: &Point3) -> Vec3 {
        todo!()
    }

    fn material(&self) -> Material {
        todo!()
    }

    fn hit(&self, ray: &Ray, time_range: &Range<f64>) -> Option<HitRecord> {
        self.list.iter()
            .filter_map(|item| item.hit(ray, time_range))
            .min_by(|l, r| l.time.partial_cmp(&r.time).unwrap_or(Ordering::Equal))

    }
}