///! Physics!
use crate::*;

pub trait Physics {
    fn physics_type(&self) -> PhysicsType;
    fn bottom_left(&self) -> Vector;
    fn width_height(&self) -> Vector;
    fn velocity(&self) -> Vector;
    fn tick(&mut self, dt: f64);
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PhysicsType {
    Static,
    Dynamic,
}
