use crate::color::{write_color, Color};
use crate::hit::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

use crate::hit::list::HittableList;
use crate::hit::sphere::Sphere;
use crate::random::sample_square;
use log;

pub struct Camera {
    aspect_ratio: f64,      // Ratio of image width over height
    image_width: i32,       // Rendered image width in pixels
    image_height: i32,      // Rendered image height
    center: Point3,         // Camera center
    pixel00_loc: Point3,    // Location of pixel 0, 0
    pixel_delta_u: Vec3,    // Offset to pixel to the right
    pixel_delta_v: Vec3,    // Offset to pixel below
    samples_per_pixel: i32, // Count of random samples for each pixel
    max_depth: i32,         // Maximum number of ray bounces into scene
    vfov: f64,              // Vertical view angle (field of view)
    look_from: Point3,      // Point camera is looking from
    look_at: Point3,        // Point camera is looking at
    vup: Vec3,              // Camera-relative "up" direction
    defocus_angle: f64,     // Variation angle of rays through each pixel
    focus_distance: f64,    // Distance from camera look_from point to plane of perfect focus
    defocus_disk_u: Vec3,   // Defocus disk horizontal radius
    defocus_disk_v: Vec3,   // Defocus disk vertical radius
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: i32,
        samples_per_pixel: i32,
        max_depth: i32,
        vfov: f64,
        look_from: Point3,
        look_at: Point3,
        vup: Vec3,
        defocus_angle: f64,
        focus_distance: f64,
    ) -> Self {
        // Calculate the image height, and ensure that it's at least 1.
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let image_height = image_height.max(1);
        let center = look_from.clone();

        // Determine viewport dimensions.
        let viewport_height = 2. * focus_distance * (vfov.to_radians() / 2.).tan();
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (&look_from - &look_at).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = &u * viewport_width;
        let viewport_v = -&v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = &viewport_u / image_width as f64;
        let pixel_delta_v = &viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            &center - &w * focus_distance - &viewport_u / 2. - &viewport_v / 2.;

        let pixel00_loc = viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = (defocus_angle / 2.).to_radians().tan() * focus_distance;
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            vfov,
            look_from,
            look_at,
            vup,
            defocus_angle,
            focus_distance,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{:} {:}\n255\n", self.image_width, self.image_height);

        for j in 0..self.image_height {
            log::info!("Scanlines remaining: {}", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::zero();
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += &Self::ray_color(&ray, &world, self.max_depth);
                }
                pixel_color /= self.samples_per_pixel as f64;
                write_color(&pixel_color);
            }
        }

        log::info!("Done!")
    }

    // Construct a camera ray originating from the defocus disk and directed at a randomly
    // sampled point around the pixel location i, j.
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = sample_square();
        let pixel_sample = &self.pixel00_loc
            + &self.pixel_delta_u * (i as f64 + offset.x)
            + &self.pixel_delta_v * (j as f64 + offset.y);

        let ray_origin = if self.defocus_angle <= 0. {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - &ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    // Returns a random point in the camera defocus disk.
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        &self.center + &self.defocus_disk_u * p.x + &self.defocus_disk_v * p.y
    }

    fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
        if depth <= 0 {
            return Color::zero(); // Black
        }

        let all_time = 0.001..f64::INFINITY;
        if let Some(record) = world.hit(ray, &all_time) {
            let material = record.material.clone();
            return if let Some(scatter) = material.scatter(ray, record) {
                Self::ray_color(&scatter.ray, world, depth - 1) * scatter.attenuation
            } else {
                Color::zero()
            };
        }

        let unit_direction = ray.direction.unit();
        let a = (unit_direction.y + 1.) * 0.5;
        let white = Color::new(1., 1., 1.);
        let blue = Color::new(0.5, 0.7, 1.0);
        white * (1. - a) + blue * a
    }
}
