use std::cmp::max;

use indicatif::{ProgressBar, ProgressStyle};

use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    ray::Ray,
    util::random_double,
    vec3::{Point, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64,  // Ratio of image width over height
    samples_per_pixel: i32, // Count of random samples for each pixel
    max_depth: i32,         // Maximum number of ray bounces into scene
    image_width: i32,       // Rendered image width in pixels
    image_height: i32,      // Rendered image height in pixels
    center: Point,          // Camera center
    pixel00_loc: Point,     // Location of pixel 0, 0
    pixel_delta_u: Vec3,    // Offset to pixel to the right
    pixel_delta_v: Vec3,    // Offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32, samples_per_pixel: i32, max_depth: i32) -> Camera {
        // Ensure that height is bigger than 1
        let image_height = max(1, (image_width as f64 / aspect_ratio) as i32);
        let center = Point::ZERO;

        // Determine viewport dimensions (assuming right-handed coordinates)
        let focal_length = 1.0;
        // Choose an arbitrary viewport height and scale the width to the desired aspect ratio
        let viewport_height = 2.0;
        // We don't use aspect_ratio here because actual aspect ratio may be different due to integer image dimensions
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Distances between pixel horizontally and vertically
        let pixel_delta_u = viewport_u / (image_width as f64);
        let pixel_delta_v = viewport_v / (image_height as f64);

        // Location of the upper left pixel
        let viewport_distance = Vec3::new(0.0, 0.0, focal_length);
        let viewport_upper_left = center - viewport_distance - (viewport_u + viewport_v) / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        let pb = ProgressBar::new(self.image_height as u64);
        pb.set_prefix("Scanlines remaining:");
        pb.set_style(ProgressStyle::with_template("{prefix} {wide_bar} {pos}/{len}").unwrap());

        // TODO: Multithreading
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(ray, self.max_depth, world)
                }
                write_color(pixel_color, self.samples_per_pixel);
            }
            pb.inc(1);
        }
        pb.finish_and_clear();
    }

    // Get a randomly sampled camera ray for the pixel at location i,j
    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let pixel_x = (i as f64) * self.pixel_delta_u;
        let pixel_y = (j as f64) * self.pixel_delta_v;
        let pixel_center = self.pixel00_loc + pixel_x + pixel_y;
        let pixel_sample = pixel_center + self.pixel_sample_square();
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    // Returns a random point in the square surrounding a pixel at the origin.
    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    fn ray_color(ray: Ray, depth: i32, world: &dyn Hittable) -> Color {
        if depth <= 0 {
            return Vec3::ZERO;
        }
        // Ignore hits that are very close to the calculated intersection point to solve the "shadow acne"
        let t_range = 0.001..f64::INFINITY;
        if let Some(hit) = world.hit(ray, t_range) {
            // Using Lambertian distribution for diffuse reflection. The reflection direction is a 
            // random vector on the unit sphere centered at P + N where P is the hit point and N 
            // is the surface normal vector.
            let reflect_direction = hit.normal + Vec3::random_unit_vector();
            return 0.5 * Camera::ray_color(Ray::new(hit.point, reflect_direction), depth - 1, world);
        }
        let unit_direction = ray.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
