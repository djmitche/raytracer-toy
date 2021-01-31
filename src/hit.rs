use crate::material::Material;
use crate::ray::Ray;
use crate::util::*;
use std::rc::Rc;

/// A hit represents a hit of a ray on a hittable
pub struct Hit {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl Hit {
    /// Create a new Hit, calculating which face is visible bsaed on the given outward normal.
    fn new_with_front_face(
        p: Point3,
        t: f64,
        r: &Ray,
        outward_normal: Vec3,
        material: Rc<dyn Material>,
    ) -> Hit {
        let front_face = r.direction.dot(&outward_normal) < 0.0;
        let mut normal = outward_normal;
        if !front_face {
            normal = -outward_normal
        }
        Hit {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

/// A hittable is a thing that rays can hit
pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

// --- Hittables

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = r.origin - self.center;
        let a = length_squared(&r.direction);
        let half_b = oc.dot(&r.direction);
        let c = length_squared(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // find the root that is within t_min..t_max
        let mut t;
        t = (-half_b - sqrtd) / a;
        if t < t_min || t_max < t {
            t = -half_b + sqrtd;
            if t < t_min || t_max < t {
                return None;
            }
        }

        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(Hit::new_with_front_face(
            p,
            t,
            r,
            outward_normal,
            self.material.clone(),
        ))
    }
}

/// Hittables groups multiple hittable objects into one
pub struct Hittables {
    hittables: Vec<Box<dyn Hittable>>,
}

impl Default for Hittables {
    fn default() -> Self {
        Self {
            hittables: Default::default(),
        }
    }
}

impl Hittables {
    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittables.push(hittable);
    }
}

impl Hittable for Hittables {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let mut hit = None;
        let mut best_t = t_max;

        for hittable in &self.hittables {
            if let Some(h) = hittable.hit(r, t_min, best_t) {
                best_t = h.t;
                hit = Some(h);
            }
        }
        hit
    }
}
