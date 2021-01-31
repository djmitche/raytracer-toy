use crate::ray::*;
use crate::util::*;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov_radians: f64,
        aspect_ratio: f64,
        aperture: f64,
    ) -> Camera {
        let h = (vfov_radians / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&vup.cross(&w));
        let v = w.cross(&u);

        let focus_distance = length(&(lookat - lookfrom));

        let origin = lookfrom;
        let horizontal = focus_distance * u * viewport_width;
        let vertical = focus_distance * v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;
        Self {
            origin,
            horizontal,
            vertical,
            u,
            v,
            w,
            lower_left_corner,
            lens_radius: aperture / 2.0,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disc();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin - offset,
        )
    }
}
