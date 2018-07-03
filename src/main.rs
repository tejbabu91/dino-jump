extern crate glutin_window;
extern crate graphics;
extern crate jump_game;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate sdl2_window;

use jump_game::app::App;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use piston_window::PistonWindow;
use sdl2_window::Sdl2Window;

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Sdl2Window window.
    let mut window: PistonWindow<Sdl2Window> =
        WindowSettings::new("spinning-square", [16 * 80, 9 * 80])
            .resizable(false)
            .vsync(false)
            .decorated(true)
            .exit_on_esc(true)
            .build()
            .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    let mut count: usize = 0;
    use std::time;
    let now = time::SystemTime::now();

    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
            count += 1;
        }
        if let Some(b) = e.button_args() {
            println!(
                "Elapsed time: {}, {}, {:?}",
                count,
                now.elapsed().unwrap().as_secs(),
                b
            );
        }
    }
}
