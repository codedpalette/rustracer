use crate::{color::Color, hittable::Hit, ray::Ray, vec3::Vec3};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit: Hit) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Metal {
        Metal {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: Ray, hit: Hit) -> Option<Scatter> {
        // Using Lambertian distribution for diffuse reflection. The reflection direction is a
        // random vector on the unit sphere centered at P + N where P is the hit point and N
        // is the surface normal vector.
        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction);
        let attenuation = self.albedo;
        Some(Scatter {
            ray: scattered,
            attenuation,
        })
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit: Hit) -> Option<Scatter> {
        let reflected = Vec3::reflect(ray_in.direction.normalize(), hit.normal);
        let scattered = Ray::new(hit.point, reflected + self.fuzz * Vec3::random_unit_vector());
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction, hit.normal) > 0.0 {
            Some(Scatter {
                ray: scattered,
                attenuation,
            })
        } else {
            None
        }
    }
}
