use crate::color::Color;
use crate::hit::hittable::HitRecord;
use crate::random::random;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Scatter {
    pub attenuation: Color,
    pub ray: Ray,
}

#[derive(Clone)]
pub enum Material {
    Dielectric(Dielectric),
    Lambertian(Lambertian),
    Metal(Metal),
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Option<Scatter> {
        match self {
            Self::Dielectric(dielectric) => dielectric.scatter(ray, hit_record),
            Self::Metal(metal) => metal.scatter(ray, hit_record),
            Self::Lambertian(lambertian) => lambertian.scatter(hit_record),
        }
    }
}

impl From<Lambertian> for Material {
    fn from(value: Lambertian) -> Self {
        Self::Lambertian(value)
    }
}

impl From<Metal> for Material {
    fn from(value: Metal) -> Self {
        Self::Metal(value)
    }
}

impl From<Dielectric> for Material {
    fn from(value: Dielectric) -> Self {
        Self::Dielectric(value)
    }
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    fn scatter(&self, hit_record: HitRecord) -> Option<Scatter> {
        let mut scatter_direction = &hit_record.normal + Vec3::random_unit();
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal
        }
        Some(Scatter {
            attenuation: self.albedo.clone(),
            ray: Ray::new(hit_record.point, scatter_direction),
        })
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = fuzz.min(1.).max(0.);
        Self { albedo, fuzz }
    }

    fn reflect(&self, ray: &Ray, hit_record: &HitRecord) -> Ray {
        let n = &hit_record.normal;
        let direction = &ray.direction - n * ray.direction.dot(n) * 2.;
        Ray::new(hit_record.point.clone(), direction)
    }

    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Option<Scatter> {
        let mut reflected = self.reflect(ray, &hit_record);
        reflected.direction = reflected.direction.unit() + Vec3::random_unit() * self.fuzz;
        if reflected.direction.dot(&hit_record.normal) > 0. {
            Some(Scatter {
                attenuation: self.albedo.clone(),
                ray: reflected,
            })
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self { refraction_index }
    }

    fn refraction_index(&self, outside: bool) -> f64 {
        if outside {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        }
    }

    // Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r = (1. - refraction_index) / (1. + refraction_index);
        let r = r * r;
        r + (1. - r) * (1. - cosine).powi(5)
    }

    fn reflect(&self, ray: &Ray, hit_record: &HitRecord) -> Ray {
        let n = &hit_record.normal;
        let direction = &ray.direction - n * ray.direction.dot(n) * 2.;
        Ray::new(hit_record.point.clone(), direction)
    }

    fn refract(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let direction = ray.direction.unit();
        let n = &hit_record.normal;
        let cos_theta = hit_record.normal.dot(&(-&direction)).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let refraction_index = self.refraction_index(hit_record.outside);
        let must_reflect = sin_theta * refraction_index > 1.;
        if must_reflect || (Self::reflectance(cos_theta, refraction_index) > random()) {
            return None;
        }

        let perpendicular = (direction + n * cos_theta) * refraction_index;
        let parallel = -n * (1. - perpendicular.length_squared()).abs().sqrt();
        Some(Ray::new(hit_record.point.clone(), perpendicular + parallel))
    }

    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Option<Scatter> {
        let refracted = self
            .refract(ray, &hit_record)
            .unwrap_or_else(|| self.reflect(ray, &hit_record));
        Some(Scatter {
            attenuation: Color::one(), // White
            ray: refracted,
        })
    }
}
