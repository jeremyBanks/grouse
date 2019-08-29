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
    pub terrain: Vec<Sprite>,
}

#[derive(Debug, Clone)]
pub struct LevelState {
    pub level: Level,
    pub player: Sprite,
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
    color: Color,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Rect {
    bottom: f64,
    left: f64,
    width: f64,
    height: f64,
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
                bottom: 0.25,
                left: 0.5,
                width: 0.125,
                height: 0.125,
            },
            color: Color::random(),
        },
        level: Level {
            terrain: vec![
                Sprite {
                    rect: Rect {
                        bottom: 0.0,
                        left: 0.0,
                        width: 1.0,
                        height: 0.0625,
                    },
                    color: Color::random(),
                },
                Sprite {
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

                for sprite in state.sprites() {
                    rectangle(
                        sprite.color.0,
                        [
                            width * sprite.rect.left,
                            height - height * (sprite.rect.bottom + sprite.rect.height),
                            width * sprite.rect.width,
                            height * sprite.rect.height,
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
                    state.player.rect.bottom += 0.1;
                }
                Key::A => {
                    state.player.rect.left -= 0.1;
                }
                Key::S => {
                    state.player.rect.bottom -= 0.1;
                }
                Key::D => {
                    state.player.rect.left += 0.1;
                }
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::W => {
                    println!("no more up");
                }
                Key::A => {
                    println!("no more left");
                }
                Key::S => {
                    println!("no more down");
                }
                Key::D => {
                    println!("no more right");
                }
                _ => {}
            }
        }
    }
}
