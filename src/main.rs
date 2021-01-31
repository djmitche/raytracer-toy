use anyhow::Result;
use geefr_ppm::Ppm;

mod vec3;

use vec3::{Color, Point3, Vec3};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;

const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);

const FHEIGHT: f64 = HEIGHT as f64;
const FWIDTH: f64 = WIDTH as f64;

const WHITE: Color = Color::new(1.0, 1.0, 1.0);
const BLUEISH: Color = Color::new(0.5, 0.7, 1.0);

struct Ray {
    orig: Point3,
    dir: Point3,
}

impl Ray {
    fn new(orig: Point3, dir: Point3) -> Self {
        Self { orig, dir }
    }

    fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.orig - *center;
    let a = r.dir.dot(&r.dir);
    let b = 2.0 * oc.dot(&r.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(r: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }
    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    WHITE * (1.0 - t) + BLUEISH * t
}

fn main() -> Result<()> {
    let mut ppm = Ppm::new(WIDTH, HEIGHT);

    let horizontal = Point3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical = Point3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner =
        ORIGIN - horizontal / 2.0 - vertical / 2.0 - Point3::new(0.0, 0.0, FOCAL_LENGTH);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f64 / FWIDTH;
            let v = y as f64 / FHEIGHT;
            let r = Ray::new(
                ORIGIN,
                lower_left_corner + horizontal * u + vertical * v - ORIGIN,
            );
            let color = ray_color(&r);
            color.set_pixel(&mut ppm, x, HEIGHT - y - 1);
        }
    }

    ppm.write("output.ppm".to_owned())?;

    Ok(())
}
