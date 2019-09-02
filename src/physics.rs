///! Physics!
use crate::*;

pub trait Physics {
    fn physics_type(&self) -> PhysicsType;
    fn bottom_left(&self) -> Vector;
    fn width_height(&self) -> Vector;
    fn velocity(&self) -> Vector;
    fn accelerate(&mut self, dt: f64) {}
}

pub enum PhysicsType {
    Static,
    Dynamic,
}
