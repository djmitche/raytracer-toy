use geefr_ppm::Ppm;
use lazy_static::lazy_static;
use nalgebra::Vector3;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

lazy_static! {
    static ref UNIFORM: Uniform<f64> = Uniform::new(0.0, 1.0);
}

pub type Vec3 = Vector3<f64>;
pub type Point3 = Vec3;
pub type Color = Vec3;

// --- Linear algebra utilities

/// Calculate the square of the length of the vector
pub fn length_squared(v: &Vec3) -> f64 {
    v.dot(v)
}

/// Calculate the length of the vector
pub fn length(v: &Vec3) -> f64 {
    length_squared(v).sqrt()
}

/// Return a vector with the same direction, but length 1.0
pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / length(v)
}

/// Set a pixel in a PPM document based on this color
pub fn set_pixel(ppm: &mut Ppm, x: usize, y: usize, c: Color) {
    ppm.set_pixel(
        x,
        y,
        (c.x * 256.0) as u8,
        (c.y * 256.0) as u8,
        (c.z * 256.0) as u8,
    )
}

pub fn near_zero(v: Vec3) -> bool {
    v.x < 1e-8 && v.y < 1e-8 && v.z < 1e-8
}

pub fn component_mult(v1: Vec3, v2: Vec3) -> Vec3 {
    Vec3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

/// Reflect v around the plane to which N is normal.  N and
/// v must be in different directions.
pub fn reflect(v: Vec3, normal: Vec3) -> Vec3 {
    v - 2.0 * v.dot(&normal) * normal
}

/// Refract v through the surface to which N is normal, using
/// the given ratio of refractive indexes
pub fn refract(v: Vec3, normal: Vec3, ratio: f64) -> Vec3 {
    let cos_theta = (-v.dot(&normal)).min(1.0);
    let r_out_perp = ratio * (v + cos_theta * normal);
    let r_out_parallel = -(1.0 - length_squared(&r_out_perp)).abs().sqrt() * normal;
    r_out_perp + r_out_parallel
}

/// Return a value uniformly sampled between 0 and 1
pub fn uniform() -> f64 {
    thread_rng().sample(*UNIFORM)
}

/// Return a random point in the unit sphere
pub fn random_in_unit_sphere() -> Point3 {
    loop {
        let p = Point3::new(
            uniform() * 2.0 - 1.0,
            uniform() * 2.0 - 1.0,
            uniform() * 2.0 - 1.0,
        );
        if length_squared(&p) < 1.0 {
            return p;
        }
    }
}

/// Return a random point *on* the unit sphere
pub fn random_on_unit_sphere() -> Point3 {
    let p = Point3::new(
        uniform() * 2.0 - 1.0,
        uniform() * 2.0 - 1.0,
        uniform() * 2.0 - 1.0,
    );
    unit_vector(&p)
}

/// Return a random point in the unit disc (with z = 0)
pub fn random_in_unit_disc() -> Point3 {
    loop {
        let p = Point3::new(uniform() * 2.0 - 1.0, uniform() * 2.0 - 1.0, 0.0);
        if length_squared(&p) < 1.0 {
            return p;
        }
    }
}

pub fn random_color() -> Color {
    Color::new(uniform(), uniform(), uniform())
}

pub fn random_color_range(min: f64, max: f64) -> Color {
    let uni = Uniform::new(min, max);
    let mut rng = thread_rng();
    Color::new(rng.sample(uni), rng.sample(uni), rng.sample(uni))
}
