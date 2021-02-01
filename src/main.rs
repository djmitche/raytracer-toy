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

const ASPECT_RATIO: f64 = 3.0 / 2.0;

const WIDTH: usize = 600;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

const VFOV_RADIANS: f64 = PI / 2.0;
const APERTURE: f64 = 0.1;

const FHEIGHT: f64 = HEIGHT as f64;
const FWIDTH: f64 = WIDTH as f64;

const SAMPLES_PER_PIXEL: usize = 200;
const MAX_RECURSION: usize = 50;

// --- Ray tracer

fn ray_color<H: Hittable>(world: &H, r: &Ray, recurse_depth: usize) -> Color {
    if recurse_depth == 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(hit) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = hit.material.scatter(r, &hit) {
            let rc = ray_color(world, &scattered, recurse_depth - 1);
            return component_mult(rc, attenuation);
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

    // ground
    let ground_mat = Rc::new(MatteFinish {
        albedo: Color::new(0.5, 0.5, 0.5),
    });

    hittables.add(Box::new(Sphere {
        center: Point3::new(0., -1000.0, 0.),
        radius: 1000.0,
        material: ground_mat,
    }));

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                (a as f64) + 0.9 * uniform(),
                0.2,
                (b as f64) + 0.9 * uniform(),
            );
            if length(&(center - Point3::new(4.0, 0.2, 0.0))) < 0.9 {
                continue;
            }
            let material: Rc<dyn Material> = match uniform() {
                x if x < 0.8 => {
                    // matte
                    let albedo = component_mult(random_color(), random_color());
                    Rc::new(MatteFinish { albedo })
                }
                x if x < 0.95 => {
                    // metal
                    let albedo = random_color_range(0.5, 1.0);
                    let fuzz = uniform() / 2.0;
                    Rc::new(MetallicFinish { albedo, fuzz })
                }
                _ => {
                    // glass
                    Rc::new(Refractive { ir: 1.5 })
                }
            };
            hittables.add(Box::new(Sphere {
                center,
                radius: 0.5,
                material,
            }));
        }
    }

    let camera = Camera::new(
        Point3::new(13., 2., 3.),
        Point3::new(3.37, 0.52, 0.78),
        Vec3::new(0., 1., 0.),
        VFOV_RADIANS,
        ASPECT_RATIO,
        APERTURE,
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
        println!("{}% finished ({} rows of {})", y * 100 / HEIGHT, y, HEIGHT);
    }

    ppm.write("output.ppm".to_owned())?;

    Ok(())
}
