use std::cmp::max;

use crate::{
    color::{write_color, Color},
    hittable::Hittable,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f64, // Ratio of image width over height
    pub image_width: i32,  // Rendered image width in pixels
    image_height: i32,     // Rendered image height in pixels
    center: Point,         // Camera center
    pixel00_loc: Point,    // Location of pixel 0, 0
    pixel_delta_u: Vec3,   // Offset to pixel to the right
    pixel_delta_v: Vec3,   // Offset to pixel below
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Camera {
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
            image_width,
            image_height,
            center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            let remaining = self.image_height - j;
            eprintln!("Scanlines remaining: {remaining}");
            for i in 0..self.image_width {
                let pixel_x = (i as f64) * self.pixel_delta_u;
                let pixel_y = (j as f64) * self.pixel_delta_v;
                let pixel_center = self.pixel00_loc + pixel_x + pixel_y;
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);

                let pixel_color = Camera::ray_color(&ray, world);
                write_color(&pixel_color);
            }
        }
        eprintln!("Done")
    }

    fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
        if let Some(hit) = world.hit(ray, 0.0..f64::INFINITY) {
            return 0.5 * (hit.normal + 1.0);
        }
        let unit_direction = ray.dir.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
