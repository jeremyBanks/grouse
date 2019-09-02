///! Internal graphics model.
use crate::*;

use rand;

#[derive(Debug)]
pub struct Rectangle {
    color: Color,
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

#[derive(Debug)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
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
        let top_right = bottom_left + (self.width_height() * size);

        Rectangle {
            color: match self.physics_type() {
                PhysicsType::Static => Color::new(0.25, 0.75, 1.0),
                PhysicsType::Dynamic => Color::new(0.75, 0.50, 0.50),
            },
            x1: bottom_left[0],
            x2: top_right[0],
            y1: height - bottom_left[1],
            y2: height - top_right[1],
        }
    }
}
