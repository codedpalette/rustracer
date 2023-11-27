mod camera;
mod color;
mod hittable;
mod material;
mod range;
mod ray;
mod sphere;
mod util;
mod vec3;
use camera::Camera;
use color::Color;
use hittable::HittableList;
use material::{Dielectric, Lambertian, Material, Metal};
use sphere::Sphere;
use util::{random_double, random_double_ranged};
use vec3::{Point, Vec3};

fn main() {
    let mut world: HittableList = vec![];

    let ground_material = Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.push(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    let base_point = Point::new(4.0, 0.2, 0.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());

            if (center - base_point).length() > 0.9 {
                let material: Box<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    Box::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_ranged(0.5, 1.0);
                    let fuzz = random_double_ranged(0.0, 0.5);
                    Box::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Box::new(Dielectric::new(1.5))
                };
                world.push(Box::new(Sphere::new(center, 0.2, material)));
            }
        }
    }

    let material1 = Box::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, material3)));

    //Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let vfov = 20.0;
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 0.6;
    let focus_dist = 10.0;
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
        focus_dist,
    );

    // TODO: Execution time
    camera.render(&world)
}
