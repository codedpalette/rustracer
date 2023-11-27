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
use material::{Dielectric, Lambertian, Metal};
use sphere::Sphere;
use vec3::{Point, Vec3};

fn main() {
    // Materials
    let ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let center = Lambertian::new(Color::new(0.1, 0.2, 0.5));
    let left = Dielectric::new(1.5);
    let right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.0);

    // World
    let sphere_ground = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, &ground);
    let sphere_center = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, &center);
    let sphere_left = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, &left);
    let sphere_left_inner = Sphere::new(Point::new(-1.0, 0.0, -1.0), -0.4, &left);
    let sphere_right = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, &right);
    let world: HittableList = vec![
        &sphere_ground,
        &sphere_center,
        &sphere_left,
        &sphere_left_inner,
        &sphere_right,
    ];

    //Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let vfov = 20.0;
    let look_from = Point::new(-2.0, 2.0, 1.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);

    let defocus_angle = 10.0;
    let focus_dist = 3.4;
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

    // TODO: execution time
    camera.render(&world)
}
