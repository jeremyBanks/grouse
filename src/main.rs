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

fn main() {
    let (width, height) = (512, 512);
    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("grouse", (width, height))
            .exit_on_esc(true)
            .graphics_api(piston_window::OpenGL::V3_2)
            .build()
            .expect("initializing graphics");

    while let Some(event) = window.next() {
        if let Some(_) = event.render_args() {
            window.draw_2d(&event, |context, graphics, _device| {
                rectangle(
                    [random(), random(), random(), random::<f32>() * 0.05],
                    [
                        random::<f64>() * 512.0,
                        random::<f64>() * 512.0,
                        random::<f64>() * 512.0,
                        random::<f64>() * 522.0,
                    ],
                    context.transform,
                    graphics,
                );
            });
        }
    }
}
