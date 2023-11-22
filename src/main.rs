mod camera;
mod color;
mod hittable;
mod range;
mod ray;
mod sphere;
mod util;
mod vec3;
use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;
use vec3::Point;

fn main() {
    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ];

    //Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let samples_per_pixel = 50;
    let max_depth = 50;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth);

    // TODO: execution time
    camera.render(&world)
}
