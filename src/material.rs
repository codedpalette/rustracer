use crate::{color::Color, hittable::Hit, ray::Ray, util::random_double, vec3::Vec3};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: Ray, hit: Hit) -> Option<Scatter>;
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
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

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

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

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct Dielectric {
    pub ir: f64, // index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    // Use Schlick's approximation for reflectance.
    fn reflectance(cosine: f64, refraction_ratio: f64) -> f64 {
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit: Hit) -> Option<Scatter> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face { 1.0 / self.ir } else { self.ir };

        let unit_direction = ray_in.direction.normalize();
        let cos_theta = f64::min(Vec3::dot(-unit_direction, hit.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double() {
            Vec3::reflect(unit_direction, hit.normal)
        } else {
            Vec3::refract(unit_direction, hit.normal, refraction_ratio)
        };
        let scattered = Ray::new(hit.point, direction);
        Some(Scatter {
            ray: scattered,
            attenuation,
        })
    }
}
