#![allow(dead_code)]
use anyhow::Result;
use geefr_ppm::Ppm;
use nalgebra::Vector3;

type Vec3 = Vector3<f64>;
type Point3 = Vec3;
type Color = Vec3;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const WIDTH: usize = 800;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const FHEIGHT: f64 = HEIGHT as f64;
const FWIDTH: f64 = WIDTH as f64;

// --- Linear algebra utilities

/// Calculate the square of the length of the vector
fn length_squared(v: &Vec3) -> f64 {
    v.dot(v)
}

/// Calculate the length of the vector
fn length(v: &Vec3) -> f64 {
    length_squared(v).sqrt()
}

/// Return a vector with the same direction, but length 1.0
fn unit_vector(v: &Vec3) -> Vec3 {
    v / length(v)
}

/// Set a pixel in a PPM document based on this color
fn set_pixel(ppm: &mut Ppm, x: usize, y: usize, c: Color) {
    ppm.set_pixel(
        x,
        y,
        (c.x * 256.0) as u8,
        (c.y * 256.0) as u8,
        (c.z * 256.0) as u8,
    )
}

// -- Ray

struct Ray {
    orig: Point3,
    dir: Point3,
}

impl Ray {
    fn new(orig: Point3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    fn at(&self, t: f64) -> Vec3 {
        self.orig + self.dir * t
    }
}

// --- Hittable

struct Hit {
    p: Point3,
    normal: Vec3,
    t: f64,
    front_face: bool,
}

impl Hit {
    fn new_with_front_face(p: Point3, t: f64, r: &Ray, outward_normal: Vec3) -> Hit {
        let front_face = r.dir.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Hit {
            p,
            normal,
            t,
            front_face,
        }
    }
}

trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}

// --- Hittables

struct Sphere {
    center: Point3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        let oc = r.orig - self.center;
        let a = length_squared(&r.dir);
        let half_b = oc.dot(&r.dir);
        let c = length_squared(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // find the root that is within t_min..t_max
        let mut root;
        root = -half_b - sqrtd;
        if root < t_min || t_max < root {
            root = -half_b + sqrtd;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root / a;
        let p = r.at(t);
        let outward_normal = (p - self.center) / self.radius;

        Some(Hit::new_with_front_face(p, t, r, outward_normal))
    }
}

struct Hittables {
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
    fn add(&mut self, hittable: Box<dyn Hittable>) {
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

// --- Ray tracer

fn ray_color<H: Hittable>(world: &H, r: &Ray) -> Color {
    if let Some(h) = world.hit(r, 0.0, 100.0) {
        let n = h.normal;
        return Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5;
    }

    let unit_direction = unit_vector(&r.dir);
    let t = 0.5 * (unit_direction.y + 1.0);

    let white: Color = Color::new(1.0, 1.0, 1.0);
    let blueish: Color = Color::new(0.5, 0.7, 1.0);
    white * (1.0 - t) + blueish * t
}

fn main() -> Result<()> {
    let mut ppm = Ppm::new(WIDTH, HEIGHT);

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);

    let horizontal = Point3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Point3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Point3::new(0.0, 0.0, FOCAL_LENGTH);

    let mut hittables = Hittables::default();
    hittables.add(Box::new(Sphere {
        center: Point3::new(0., 0., -1.0),
        radius: 0.5,
    }));
    hittables.add(Box::new(Sphere {
        center: Point3::new(1., -100.5, -1.0),
        radius: 100.,
    }));

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f64 / FWIDTH;
            let v = y as f64 / FHEIGHT;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let color = ray_color(&hittables, &r);
            set_pixel(&mut ppm, x, HEIGHT - y - 1, color);
        }
    }

    ppm.write("output.ppm".to_owned())?;

    Ok(())
}
