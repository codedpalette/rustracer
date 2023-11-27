use std::ops::Range;

use crate::{
    hittable::{Hit, Hittable},
    material::Material,
    range::Interval,
    ray::Ray,
    vec3::{Point, Vec3},
};

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<Hit> {
        // Define coefficients of a quadratic equation for `t` in order to
        // determine ray-sphere intersection points
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(oc, ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        };
        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if !t_range.surrounds(root) {
            root = (-half_b + sqrtd) / a;
            if !t_range.surrounds(root) {
                return None;
            }
        }

        let t = root;
        let hit_point = ray.at(t);
        let outward_normal = (hit_point - self.center) / self.radius;
        Some(Hit::new(ray, t, outward_normal, self.material.as_ref()))
    }
}
