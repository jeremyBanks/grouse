///! Internal graphics model.
use crate::*;

use rand;

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub color: Color,
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, 1.0]
    }

    pub fn random() -> Color {
        Color::new(rand::random(), rand::random(), rand::random())
    }
}

pub trait ToRectangle: Physics {
    fn to_rectangle(&self, width: f64, height: f64) -> Rectangle {
        let size = width.min(height);

        let bottom_left = self.bottom_left() * size;
        let width_height = self.width_height() * size;

        Rectangle {
            color: match self.physics_type() {
                PhysicsType::Static => Color::new(0.25, 0.75, 1.0),
                PhysicsType::Dynamic => Color::new(0.75, 0.50, 0.50),
            },
            left: bottom_left.x(),
            top: height - bottom_left.y() - width_height.y(),
            width: width_height.x(),
            height: width_height.y(),
        }
    }
}

impl ToRectangle for Terrain {}
impl ToRectangle for Player {}
