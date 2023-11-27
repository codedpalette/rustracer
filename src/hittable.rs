use std::ops::Range;

use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Copy, Clone)]
pub struct Hit<'a> {
    pub point: Point,               // hit point coordinates
    pub normal: Vec3,               // surface normal at hit point
    pub t: f64,                     // distance along the ray from ray's origin to hit point
    pub front_face: bool,           // if true, hit ocurred from the front face side
    pub material: &'a dyn Material, // material of the hit surface
}

impl<'a> Hit<'a> {
    // Assume that outward_normal is normalized
    pub fn new(ray: Ray, t: f64, outward_normal: Vec3, material: &dyn Material) -> Hit {
        let point = ray.at(t);
        let front_face = Vec3::dot(ray.direction, outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Hit {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<Hit>;
}

pub type HittableList = Vec<Box<dyn Hittable>>;

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_range: Range<f64>) -> Option<Hit> {
        let mut hit_anything = None;
        let mut closest_so_far = t_range.end;

        for object in self.iter() {
            if let Some(hit) = object.hit(ray, t_range.start..closest_so_far) {
                closest_so_far = hit.t;
                hit_anything = Some(hit);
            }
        }
        hit_anything
    }
}
