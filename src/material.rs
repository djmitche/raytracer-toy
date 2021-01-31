use crate::hit::Hit;
use crate::ray::Ray;
use crate::util::*;

/// Material properties
pub trait Material {
    /// scatter the given ray with the given hit, returning
    /// a pair (attenuation, scattered), or None if the ray
    /// was absorbed.
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)>;
}

pub struct MatteFinish {
    pub albedo: Color,
}

impl Material for MatteFinish {
    fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let mut scatter_dir = hit.normal + random_on_unit_sphere();
        if near_zero(scatter_dir) {
            scatter_dir = hit.normal;
        }
        let scatter = Ray::new(hit.p, scatter_dir);
        Some((self.albedo, scatter))
    }
}

pub struct MetallicFinish {
    pub albedo: Color,
    /// Fuzzing of the reflection
    pub fuzz: f64,
}

impl Material for MetallicFinish {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let reflected = reflect(unit_vector(&ray.direction), hit.normal);
        let scatter = Ray::new(hit.p, reflected + self.fuzz * random_in_unit_sphere());
        Some((self.albedo, scatter))
    }
}

pub struct Refractive {
    /// index of refraction
    pub ir: f64,
}

/// Schlick's approximation for reflectance
fn reflectance(cosine: f64, reflective_index: f64) -> f64 {
    let mut r0 = (1.0 - reflective_index) / (1.0 + reflective_index);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

impl Material for Refractive {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Color, Ray)> {
        let mut refraction_ratio = self.ir;
        if hit.front_face {
            refraction_ratio = 1.0 / refraction_ratio;
        }
        let unit_direction = unit_vector(&ray.direction);

        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut cannot_refract = refraction_ratio * sin_theta > 1.0;
        cannot_refract = cannot_refract || reflectance(cos_theta, refraction_ratio) > uniform();

        let direction = if cannot_refract {
            reflect(unit_direction, hit.normal)
        } else {
            refract(unit_direction, hit.normal, refraction_ratio)
        };
        let scattered = Ray::new(hit.p, direction);
        Some((Color::new(1., 1., 1.), scattered))
    }
}
