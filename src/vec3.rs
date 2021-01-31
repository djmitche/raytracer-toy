use geefr_ppm::Ppm;
use std::ops::*;

// TODO: use https://docs.rs/nalgebra/0.24.1/nalgebra/index.html

#[derive(Copy, Debug, PartialEq)]
pub(crate) struct Vec3([f64; 3]);

impl Vec3 {
    pub(crate) const fn new(a: f64, b: f64, c: f64) -> Self {
        Vec3([a, b, c])
    }

    pub(crate) fn x(&self) -> f64 {
        self.0[0]
    }

    pub(crate) fn y(&self) -> f64 {
        self.0[1]
    }

    pub(crate) fn z(&self) -> f64 {
        self.0[2]
    }

    pub(crate) fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub(crate) fn length_squared(&self) -> f64 {
        self.dot(self)
    }

    /// Return a vector with the same direction but length 0
    pub(crate) fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub(crate) fn dot(&self, other: &Vec3) -> f64 {
        self.0[0] * other.0[0] + self.0[1] * other.0[1] + self.0[2] * other.0[2]
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3([0.0, 0.0, 0.0])
    }
}

impl Clone for Vec3 {
    fn clone(&self) -> Self {
        Vec3([self.0[0], self.0[1], self.0[2]])
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3([
            self.0[0] + other.0[0],
            self.0[1] + other.0[1],
            self.0[2] + other.0[2],
        ])
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3([
            self.0[0] - other.0[0],
            self.0[1] - other.0[1],
            self.0[2] - other.0[2],
        ])
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, other: f64) -> Vec3 {
        Vec3([self.0[0] / other, self.0[1] / other, self.0[2] / other])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f64) -> Vec3 {
        Vec3([self.0[0] * other, self.0[1] * other, self.0[2] * other])
    }
}

macro_rules! vec3_wrapper_binop {
    ($type:ident, $trait:ty, $fn:ident) => {
        impl $trait for $type {
            type Output = $type;
            fn $fn(self, other: Self) -> Self::Output {
                $type(Vec3::$fn(self.0, other.0))
            }
        }
    };
}

macro_rules! vec3_wrapper_binop_f64 {
    ($type:ident, $trait:ident, $fn:ident) => {
        impl $trait<f64> for $type {
            type Output = $type;
            fn $fn(self, other: f64) -> Self::Output {
                $type(Vec3::$fn(self.0, other))
            }
        }
    };
}

macro_rules! vec3_type {
    ($type:ident) => {
        #[derive(Copy, Debug, PartialEq)]
        pub(crate) struct $type(pub(crate) Vec3);

        impl Default for $type {
            fn default() -> Self {
                $type(Default::default())
            }
        }

        impl Clone for $type {
            fn clone(&self) -> Self {
                $type(self.0.clone())
            }
        }

        impl Deref for $type {
            type Target = Vec3;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        vec3_wrapper_binop!($type, Add, add);
        vec3_wrapper_binop!($type, Sub, sub);
        vec3_wrapper_binop_f64!($type, Div, div);
        vec3_wrapper_binop_f64!($type, Mul, mul);
    };
}

pub(crate) type Point3 = Vec3;

vec3_type!(Color);

impl Color {
    pub(crate) const fn new(r: f64, g: f64, b: f64) -> Self {
        Color(Vec3::new(r, g, b))
    }

    pub(crate) fn r(&self) -> f64 {
        self.0 .0[0]
    }

    pub(crate) fn g(&self) -> f64 {
        self.0 .0[1]
    }

    pub(crate) fn b(&self) -> f64 {
        self.0 .0[2]
    }

    pub(crate) fn set_pixel(&self, ppm: &mut Ppm, x: usize, y: usize) {
        ppm.set_pixel(
            x,
            y,
            (self.r() * 256.0) as u8,
            (self.g() * 256.0) as u8,
            (self.b() * 256.0) as u8,
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_vec3() {
        assert_eq!(
            Vec3::default() + Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(1.0, 2.0, 3.0)
        );
    }

    #[test]
    fn add_point3() {
        assert_eq!(
            Point3::default() + Point3::new(1.0, 2.0, 3.0),
            Point3::new(1.0, 2.0, 3.0)
        );
    }
}
