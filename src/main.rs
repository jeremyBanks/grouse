use nalgebra;
use piston_window::{
    self,
    clear,
    rectangle,
    Button,
    Key,
    PressEvent,
    ReleaseEvent,
    RenderEvent,
};

type Vector2 = nalgebra::Vector2<f64>;
trait Vector2Utils {
    fn x(&self) -> f64;
    fn set_x(&mut self, x: f64);
    fn add_x(&mut self, x: f64);
    fn y(&self) -> f64;
    fn set_y(&mut self, y: f64);
    fn add_y(&mut self, y: f64);
}
impl Vector2Utils for Vector2 {
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
type Color = [f32; 4];

pub mod colors {
    use super::Color;

    pub fn black() -> Color {
        [0.0, 0.0, 0.0, 1.0]
    }

    pub fn white() -> Color {
        [1.0, 1.0, 1.0, 1.0]
    }

    pub fn transparent() -> Color {
        [0.0, 0.0, 0.0, 0.0]
    }

    pub fn random() -> Color {
        [rand::random(), rand::random(), rand::random(), 0.75]
    }
}

#[derive(Debug, Clone)]
pub struct Level {
    pub terrain: Vec<Sprite>,
}

#[derive(Debug, Clone)]
pub struct LevelState {
    pub level: Level,
    pub player: Sprite,
    pub input: Input,
}

#[derive(Debug, Clone)]
pub struct Input {
    pub acceleration: Vector2,
}

impl LevelState {
    pub fn sprites(&self) -> Vec<Sprite> {
        let mut sprites = Vec::new();

        for sprite in self.level.terrain.iter() {
            sprites.push(*sprite);
        }

        sprites.push(self.player);

        sprites
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sprite {
    rect: Rect,
    velocity: Vector2,
    color: Color,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    bottom_left: Vector2,
    dimensions: Vector2,
}

fn main() {
    let (width, height) = (512.0, 512.0);
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Grouse", (width as u32, height as u32))
            .exit_on_esc(true)
            .resizable(false)
            .graphics_api(piston_window::OpenGL::V3_2)
            .build()
            .expect("initializing graphics");

    let mut state = LevelState {
        player: Sprite {
            rect: Rect {
                bottom_left: Vector2::new(0.25, 0.5),
                dimensions: Vector2::new(0.125, 0.125),
            },
            color: colors::random(),
            velocity: Vector2::new(0.0, 0.0),
        },
        input: Input {
            acceleration: Vector2::new(0.0, 0.0),
        },
        level: Level {
            terrain: vec![
                Sprite {
                    rect: Rect {
                        bottom_left: Vector2::new(0.0, 0.0),
                        dimensions: Vector2::new(1.0, 0.05),
                    },
                    color: colors::random(),
                    velocity: Vector2::new(0.0, 0.0),
                },
                Sprite {
                    rect: Rect {
                        bottom_left: Vector2::new(0.0, 0.05),
                        dimensions: Vector2::new(0.5, 0.05),
                    },
                    color: colors::random(),
                    velocity: Vector2::new(0.0, 0.0),
                },
            ],
        },
    };

    while let Some(event) = window.next() {
        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                // clear(colors::black(), graphics);
                rectangle(
                    [0.0, 0.0, 0.0, 0.25],
                    [0.0, 0.0, width, height],
                    context.transform,
                    graphics,
                );

                let dt = 1.0 / 60.0;
                if state.input.acceleration.norm() > 0.0 {
                    // unbounded acceleration
                    state.player.velocity += state.input.acceleration * dt;
                } else if state.player.velocity.norm() > 0.1 {
                    // kinetic friction
                    state.player.velocity -= 2.0 * state.player.velocity * dt;
                } else if state.player.velocity.norm() > 0.0 {
                    // static friction
                    state.player.velocity *= 0.0;
                } else {
                    // lol!
                    state
                        .player
                        .velocity
                        .set_x((rand::random::<f64>() - 0.5) * 16.0);
                    state
                        .player
                        .velocity
                        .set_y((rand::random::<f64>() - 0.5) * 16.0);
                }

                if state.player.velocity.norm() > 1.5 {
                    state.player.velocity *= 1.5 / state.player.velocity.norm()
                }
                state.player.rect.bottom_left += state.player.velocity * dt;

                if state.player.rect.bottom_left.x() < -0.5 {
                    state.player.rect.bottom_left.add_x(2.0);
                }
                if state.player.rect.bottom_left.x() > 1.5 {
                    state.player.rect.bottom_left.add_x(-2.0);
                }
                if state.player.rect.bottom_left.y() < -0.5 {
                    state.player.rect.bottom_left.add_y(2.0);
                }
                if state.player.rect.bottom_left.y() > 1.5 {
                    state.player.rect.bottom_left.add_y(-2.0);
                }

                for sprite in state.sprites() {
                    rectangle(
                        sprite.color,
                        [
                            width * sprite.rect.bottom_left.x(),
                            height
                                - height
                                    * (sprite.rect.bottom_left.y() + sprite.rect.dimensions.y()),
                            width * sprite.rect.dimensions.x(),
                            height * sprite.rect.dimensions.y(),
                        ],
                        context.transform,
                        graphics,
                    );
                }
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => {
                    state.input.acceleration.set_y(4.0);
                }
                Key::A => {
                    state.input.acceleration.set_x(-4.0);
                }
                Key::S => {
                    state.input.acceleration.set_y(-4.0);
                }
                Key::D => {
                    state.input.acceleration.set_x(4.0);
                }
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::W => {
                    if state.input.acceleration.y() > 0.0 {
                        state.input.acceleration.set_y(0.0);
                    }
                }
                Key::A => {
                    if state.input.acceleration.x() < 0.0 {
                        state.input.acceleration.set_x(0.0);
                    }
                }
                Key::S => {
                    if state.input.acceleration.y() < 0.0 {
                        state.input.acceleration.set_y(0.0);
                    }
                }
                Key::D => {
                    if state.input.acceleration.x() > 0.0 {
                        state.input.acceleration.set_x(0.0);
                    }
                }
                _ => {}
            }
        }
    }
}
