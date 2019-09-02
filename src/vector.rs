///! The 2D vector we use, rexported from nalgebra with a utility trait added.
use crate::*;

use nalgebra;

pub type Vector = nalgebra::Vector2<f64>;

pub trait VectorUtils {
    fn zero() -> Self;
    fn x(&self) -> f64;
    fn set_x(&mut self, x: f64);
    fn add_x(&mut self, x: f64);
    fn y(&self) -> f64;
    fn set_y(&mut self, y: f64);
    fn add_y(&mut self, y: f64);
}

impl VectorUtils for Vector {
    fn zero() -> Vector {
        Vector::new(0.0, 0.0)
    }

    fn x(&self) -> f64 {
        self[0]
    }

    fn set_x(&mut self, x: f64) {
        self[0] = x;
    }

    fn add_x(&mut self, x: f64) {
        self[0] += x;
    }

    fn y(&self) -> f64 {
        self[1]
    }

    fn set_y(&mut self, y: f64) {
        self[1] = y;
    }

    fn add_y(&mut self, y: f64) {
        self[1] += y;
    }
}
