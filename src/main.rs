use std::cell::RefCell;

use piston_window::{
    clear,
    rectangle,
    OpenGL,
    PistonWindow,
    PressEvent,
    ReleaseEvent,
    RenderEvent,
    WindowSettings,
};
use rand::random;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color([f32; 4]);

impl Color {
    pub fn black() -> Color {
        Color([0.0, 0.0, 0.0, 1.0])
    }

    pub fn white() -> Color {
        Color([1.0, 1.0, 1.0, 1.0])
    }

    pub fn transparent() -> Color {
        Color([0.0, 0.0, 0.0, 0.0])
    }

    pub fn random() -> Color {
        Color([random(), random(), random(), 1.0])
    }
}

#[derive(Debug, Clone)]
pub struct Level {
    pub terrain: Vec<Terrain>,
}

pub struct LevelState {
    pub level: Level,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Coordinate {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    bottom: f64,
    left: f64,
    width: f64,
    height: f64,
}

#[derive(Debug, Clone)]
pub struct Terrain {
    rect: Rect,
    color: Color,
}

fn main() {
    let (width, height) = (512.0, 512.0);
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Grouse", (512, 512))
            .exit_on_esc(true)
            .graphics_api(piston_window::OpenGL::V3_2)
            .build()
            .expect("initializing graphics");

    let state = LevelState {
        level: Level {
            terrain: vec![
                Terrain {
                    rect: Rect {
                        bottom: 0.0,
                        left: 0.0,
                        width: 1.0,
                        height: 0.0625,
                    },
                    color: Color::random(),
                },
                Terrain {
                    rect: Rect {
                        bottom: 0.0625,
                        left: 0.0,
                        width: 0.5,
                        height: 0.0625,
                    },
                    color: Color::random(),
                },
            ],
        },
    };

    while let Some(event) = window.next() {
        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear(Color::black().0, graphics);
                for terrain in state.level.terrain.iter() {
                    rectangle(
                        terrain.color.0,
                        [
                            width * terrain.rect.left,
                            height - height * (terrain.rect.bottom + terrain.rect.height),
                            width * terrain.rect.width,
                            height * terrain.rect.height,
                        ],
                        context.transform,
                        graphics,
                    );
                }
            });
        }
    }
}
