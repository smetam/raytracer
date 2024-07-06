use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::hit::list::HittableList;
use raytracer::hit::sphere::Sphere;
use raytracer::materials::{Dielectric, Lambertian, Metal};
use raytracer::random::random;
use raytracer::vec3::{Point3, Vec3};

fn build_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.0,
        ground_material.into(),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point3::new(a as f64 + 0.9 * random(), 0.2, b as f64 + 0.9 * random());

            if (&center - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                let sphere_material = if choose_mat < 0.8 {
                    // diffuse
                    let albedo: Color = Color::random() * Color::random();
                    Lambertian::new(albedo).into()
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo: Color = Color::random_range(&(0.5..1.));
                    let fuzz = random() / 2.;
                    Metal::new(albedo, fuzz).into()
                } else {
                    // glass
                    Dielectric::new(1.5).into()
                };
                world.add(Sphere::new(center, 0.2, sphere_material));
            }
        }
    }

    let material1 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(0., 1., 0.), 1.0, material1.into()));

    let material2 = Lambertian::new(Color::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(-4., 1., 0.), 1.0, material2.into()));

    let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4., 1., 0.), 1.0, material3.into()));
    world
}

fn main() {
    env_logger::init();
    // World
    let world = build_world();

    // Render
    let aspect_ratio = 16. / 9.;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.;
    let look_from = Point3::new(13., 2., 3.);
    let look_at = Point3::zero();
    let vup = Vec3::new(0., 1., 0.);

    let defocus_angle = 0.6;
    let focus_distance = 10.;
    let camera = Camera::new(
        aspect_ratio,
        image_width,
        samples_per_pixel,
        max_depth,
        vfov,
        look_from,
        look_at,
        vup,
        defocus_angle,
        focus_distance,
    );
    camera.render(&world)
}
