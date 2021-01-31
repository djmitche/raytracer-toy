use anyhow::Result;
use geefr_ppm::Ppm;
use std::f64::consts::PI;
use std::rc::Rc;

mod camera;
mod hit;
mod material;
mod ray;
mod util;

pub use crate::camera::*;
pub use crate::hit::*;
pub use crate::material::*;
pub use crate::ray::*;
pub use crate::util::*;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const WIDTH: usize = 1200;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

const VFOV_RADIANS: f64 = PI / 4.0;

const FHEIGHT: f64 = HEIGHT as f64;
const FWIDTH: f64 = WIDTH as f64;

const SAMPLES_PER_PIXEL: usize = 10;
const MAX_RECURSION: usize = 10;

// --- Ray tracer

fn ray_color<H: Hittable>(world: &H, r: &Ray, recurse_depth: usize) -> Color {
    if recurse_depth == 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            let rc = ray_color(world, &scattered, recurse_depth - 1);
            // TODO: elementwise multiplication method
            return Color::new(
                rc.x * attenuation.x,
                rc.y * attenuation.y,
                rc.z * attenuation.z,
            );
        }
    }

    // draw the background
    let unit_direction = unit_vector(&r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    let white: Color = Color::new(1.0, 1.0, 1.0);
    let bluish: Color = Color::new(0.5, 0.5, 1.0);
    white * (1.0 - t) + bluish * t
}

fn main() -> Result<()> {
    let mut ppm = Ppm::new(WIDTH, HEIGHT);

    let mut hittables = Hittables::default();
    hittables.add(Box::new(Sphere {
        center: Point3::new(0., 0., -1.0),
        radius: 0.5,
        material: Rc::new(MatteFinish {
            albedo: Color::new(1.0, 1.0, 1.0),
        }),
    }));
    hittables.add(Box::new(Sphere {
        center: Point3::new(-1., 0., -1.0),
        radius: 0.5,
        material: Rc::new(Refractive { ir: 1.5 }),
    }));
    hittables.add(Box::new(Sphere {
        center: Point3::new(1., 0., -1.0),
        radius: 0.5,
        material: Rc::new(MetallicFinish {
            albedo: Color::new(0.4, 0.4, 1.0),
            fuzz: 0.1,
        }),
    }));
    hittables.add(Box::new(Sphere {
        center: Point3::new(1., -100.5, -1.0),
        radius: 100.,
        material: Rc::new(MatteFinish {
            albedo: Color::new(0.1, 0.7, 0.1),
        }),
    }));

    let camera = Camera::new(
        Point3::new(3., 3., 2.),
        Point3::new(0., 0., -1.),
        Vec3::new(0., 1., 0.),
        VFOV_RADIANS,
        ASPECT_RATIO,
        2.0,
    );

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (x as f64 + uniform()) / FWIDTH;
                let v = (y as f64 + uniform()) / FHEIGHT;
                let r = camera.get_ray(u, v);
                color += ray_color(&hittables, &r, MAX_RECURSION);
            }
            color = (color / (SAMPLES_PER_PIXEL as f64)).map(|x| x.sqrt());
            set_pixel(&mut ppm, x, HEIGHT - y - 1, color);
        }
    }

    ppm.write("output.ppm".to_owned())?;

    Ok(())
}
