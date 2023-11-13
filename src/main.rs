mod color;
mod range;
mod ray;
mod surface;
mod vec3;
use color::write_color;
use color::Color;
use ray::Ray;
use std::cmp::max;
use surface::hittable::Hittable;
use surface::hittable::HittableList;
use surface::sphere::Sphere;
use vec3::{Point, Vec3};

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    if let Some(hit) = world.hit(ray, 0.0..f64::INFINITY) {
        return 0.5 * (hit.normal + 1.0);
    }
    let unit_direction = ray.dir.normalize();
    let a = 0.5 * (unit_direction.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    // Ensure that height is bigger than 1
    let image_height = max(1, (image_width as f64 / aspect_ratio) as i32);

    // World
    let world: HittableList = vec![
        Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)),
    ];

    // Camera (assuming right-handed coordinates)
    let focal_length = 1.0;
    // Choose an arbitrary viewport height and scale the width to the desired aspect ratio
    let viewport_height = 2.0;
    // We don't use aspect_ratio here because actual aspect ratio may be different due to integer image dimensions
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point::ZERO;

    // Vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Distances between pixel horizontally and vertically
    let pixel_delta_u = viewport_u / (image_width as f64);
    let pixel_delta_v = viewport_v / (image_height as f64);

    // Location of the upper left pixel
    let viewport_distance = Vec3::new(0.0, 0.0, focal_length);
    let viewport_upper_left = camera_center - viewport_distance - (viewport_u + viewport_v) / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        let remaining = image_height - j;
        eprintln!("Scanlines remaining: {remaining}");
        for i in 0..image_width {
            let pixel_x = (i as f64) * pixel_delta_u;
            let pixel_y = (j as f64) * pixel_delta_v;
            let pixel_center = pixel00_loc + pixel_x + pixel_y;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&ray, &world);
            write_color(&pixel_color);
        }
    }
    eprintln!("Done")
}
