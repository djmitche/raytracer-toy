use ::rand::distributions::Uniform;
use ::rand::Rng;
use ::rand::{thread_rng, Rng};

use crate::util::*;

pub struct Rand<R: Rng> {
    rng: R,
    uniform: Uniform<f64>,
}

impl<R: Rng> Rand<R> {
    pub fn new(rng: R) -> Self {
        Self {
            rng,
            uniform: Uniform::new(0.0, 1.0),
        }
    }
}
