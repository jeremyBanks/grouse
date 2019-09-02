///! Event loop, initialization, glue.
///!
///! Provides input to our game state, and renders out its internal graphics model.
use crate::*;

#[allow(unused_imports)]
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

use std::time::{
    Duration,
    SystemTime,
};

#[derive(Debug)]
pub struct Input {
    /// The direction of input.
    pub acceleration: Vector,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (width, height) = (512.0, 512.0);
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Grouse", (width as u32, height as u32))
            .exit_on_esc(true)
            .resizable(false)
            .graphics_api(piston_window::OpenGL::V3_2)
            .build()?;

    let mut world = World::new();

    let mut input = Input {
        acceleration: Vector::zero(),
    };

    let time_step_secs = 1.0f64 / 240.0;
    let time_step_duration = Duration::from_secs_f64(time_step_secs);

    let mut last_t = SystemTime::now();
    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => {
                    input.acceleration.set_y(1.0);
                }
                Key::A => {
                    input.acceleration.set_x(-1.0);
                }
                Key::S => {
                    input.acceleration.set_y(-1.0);
                }
                Key::D => {
                    input.acceleration.set_x(1.0);
                }
                _ => {}
            }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::W => {
                    if input.acceleration.y() > 0.0 {
                        input.acceleration.set_y(0.0);
                    }
                }
                Key::A => {
                    if input.acceleration.x() < 0.0 {
                        input.acceleration.set_x(0.0);
                    }
                }
                Key::S => {
                    if input.acceleration.y() < 0.0 {
                        input.acceleration.set_y(0.0);
                    }
                }
                Key::D => {
                    if input.acceleration.x() > 0.0 {
                        input.acceleration.set_x(0.0);
                    }
                }
                _ => {}
            }
        }

        {
            // Tick world as many times as neccessary.
            let t = SystemTime::now();
            match t.duration_since(last_t) {
                Ok(dt) => {
                    let mut dt = dt.as_secs_f64();
                    while dt > time_step_secs {
                        world.tick(time_step_secs, &input);
                        dt -= time_step_secs;
                        last_t += time_step_duration;
                    }
                }
                Err(error) => {
                    info!(
                        "SystemTime::now() went backwards by {:?}.",
                        error.duration()
                    );
                    last_t = t;
                }
            }
        }

        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear(Color::black().to_array(), graphics);

                for rectangle in world.rectangles(width, height) {
                    piston_window::rectangle(
                        rectangle.color.to_array(),
                        [
                            rectangle.left,
                            rectangle.top,
                            rectangle.width,
                            rectangle.height,
                        ],
                        context.transform,
                        graphics,
                    );
                }
            });
        }
    }

    Ok(())
}
