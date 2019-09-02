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
    Instant,
};

#[derive(Debug)]
pub struct Input {
    /// A zero or unit vector indicating the direction of input.
    acceleration: Vector,
}

pub fn run() {
    let (width, height) = (512.0, 512.0);
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Grouse", (width as u32, height as u32))
            .exit_on_esc(true)
            .resizable(false)
            .graphics_api(piston_window::OpenGL::V3_2)
            .build()
            .expect("initializing graphics");

    // let mut state = LevelState {
    //     player: Sprite {
    //         rect: Rect {
    //             bottom_left: Vector::new(0.25, 0.5),
    //             dimensions: Vector::new(0.125, 0.125),
    //         },
    //         color: colors::random(),
    //         velocity: Vector::new(0.0, 0.0),
    //     },
    //     input: Input {
    //         acceleration: Vector::new(0.0, 0.0),
    //     },
    //     level: Level {
    //         terrain: vec![
    //             // Sprite {
    //             //     rect: Rect {
    //             //         bottom_left: Vector::new(0.0, 0.0),
    //             //         dimensions: Vector::new(1.0, 0.05),
    //             //     },
    //             //     color: colors::random(),
    //             //     velocity: Vector::new(0.0, 0.0),
    //             // },
    //             // Sprite {
    //             //     rect: Rect {
    //             //         bottom_left: Vector::new(0.0, 0.05),
    //             //         dimensions: Vector::new(0.5, 0.05),
    //             //     },
    //             //     color: colors::random(),
    //             //     velocity: Vector::new(0.0, 0.0),
    //             // },
    //         ],
    //     },
    // };

    let mut last_t = Instant::now();
    while let Some(event) = window.next() {
        let t = Instant::now();
        let dt = t.duration_since(last_t).as_secs_f64();
        last_t = t;
        // we should really do fixed-duration physics steps that we fill to the current t

        if let Some(_render) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                // // let x: piston_window::Context = context;

                // // let y: Box<dyn piston_window::Graphics> = Box::new(graphics);

                // clear(colors::black(), graphics);

                // if state.input.acceleration.norm() > 0.0 {
                //     // unbounded acceleration
                //     state.player.velocity += state.input.acceleration * dt;
                // } else if state.player.velocity.norm() > 0.25 {
                //     // kinetic friction
                //     state.player.velocity -= state.player.velocity * dt;
                // } else if state.player.velocity.norm() > 0.0 {
                //     // static friction
                //     state.player.velocity *= 0.0;
                // } else {
                //     // lol!
                //     state.player.color = colors::random();
                //     state
                //         .player
                //         .velocity
                //         .set_x((rand::random::<f64>() - 0.5) * 128.0);
                //     state
                //         .player
                //         .velocity
                //         .set_y((rand::random::<f64>() - 0.5) * 128.0);
                // }

                // if state.player.velocity.norm() > 1.5 {
                //     state.player.velocity *= 1.5 / state.player.velocity.norm()
                // }
                // state.player.rect.bottom_left += state.player.velocity * dt;

                // if state.player.rect.bottom_left.x() < -0.25 {
                //     state.player.rect.bottom_left.add_x(1.50);
                // }
                // if state.player.rect.bottom_left.x() > 1.5 {
                //     state.player.rect.bottom_left.add_x(-1.50);
                // }
                // if state.player.rect.bottom_left.y() < -0.25 {
                //     state.player.rect.bottom_left.add_y(1.50);
                // }
                // if state.player.rect.bottom_left.y() > 1.5 {
                //     state.player.rect.bottom_left.add_y(-1.50);
                // }

                // for sprite in state.sprites() {
                //     rectangle(
                //         sprite.color,
                //         [
                //             width * sprite.rect.bottom_left.x(),
                //             height
                //                 - height
                //                     * (sprite.rect.bottom_left.y() + sprite.rect.dimensions.y()),
                //             width * sprite.rect.dimensions.x(),
                //             height * sprite.rect.dimensions.y(),
                //         ],
                //         context.transform,
                //         graphics,
                //     );
                // }
            });
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            // match key {
            //     Key::W => {
            //         state.input.acceleration.set_y(4.0);
            //     }
            //     Key::A => {
            //         state.input.acceleration.set_x(-4.0);
            //     }
            //     Key::S => {
            //         state.input.acceleration.set_y(-4.0);
            //     }
            //     Key::D => {
            //         state.input.acceleration.set_x(4.0);
            //     }
            //     _ => {}
            // }
        }

        if let Some(Button::Keyboard(key)) = event.release_args() {
            //     match key {
            //         Key::W => {
            //             if state.input.acceleration.y() > 0.0 {
            //                 state.input.acceleration.set_y(0.0);
            //             }
            //         }
            //         Key::A => {
            //             if state.input.acceleration.x() < 0.0 {
            //                 state.input.acceleration.set_x(0.0);
            //             }
            //         }
            //         Key::S => {
            //             if state.input.acceleration.y() < 0.0 {
            //                 state.input.acceleration.set_y(0.0);
            //             }
            //         }
            //         Key::D => {
            //             if state.input.acceleration.x() > 0.0 {
            //                 state.input.acceleration.set_x(0.0);
            //             }
            //         }
            //         _ => {}
            //     }
        }
    }
}
