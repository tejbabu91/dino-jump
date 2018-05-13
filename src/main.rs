extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    rotation: f64, // Rotation for the square.
    obstacles: Vec<Object>,
}

struct Object {
    width: u32,
    height: u32,
    pos_x: i32,
    pos_y: i32,
}

impl Object {
    fn move_by_px_x(&mut self, offset: i32) {
        self.pos_x += offset;
    }

    fn new() -> Object {
        Object {
            width: 10,
            height: 20,
            pos_x: 0,
            pos_y: 0,
        }
    }

    fn get_draw_coordinates(&self) -> (f64, f64, f64, f64) {
        (
            self.pos_x as f64,
            self.pos_y as f64,
            (self.pos_x + self.width as i32) as f64,
            (self.pos_y + self.height as i32) as f64,
        )
    }
}

impl App {
    fn new(gl: GlGraphics) -> App {
        App {
            gl: gl,
            rotation: 0.0,
            obstacles: vec![],
        }
    }

    fn add_object(&mut self, o: Object) {
        self.obstacles.push(o)
    }

    fn draw_line(&mut self, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);
        let line = graphics::rectangle::rectangle_by_corners(
            0.0,
            (args.height * 2 / 3) as f64,
            (args.width) as f64,
            (args.height * 2 / 3 + 10) as f64,
        );

        self.gl.draw(args.viewport(), |c, gl| {
            // graphics::clear(WHITE, gl);

            let transform = c.transform; //.trans(x, y);

            graphics::rectangle(BLACK, line, transform, gl);
        });
    }

    fn draw_obstacles(&mut self, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
        for obj in &mut self.obstacles {
            let (x0, y0, x1, y1) = obj.get_draw_coordinates();
            let obstacle = graphics::rectangle::rectangle_by_corners(x0, y0, x1, y1);
            self.gl.draw(args.viewport(), |c, gl| {
                // graphics::clear(WHITE, gl);

                let transform = c.transform; //.trans(x, y);

                graphics::rectangle(BLACK, obstacle, transform, gl);
            });
            obj.move_by_px_x(-10);
        }

        if !self.obstacles.is_empty() && self.obstacles[0].pos_x < 0 {
            self.obstacles.remove(0);
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c.transform.trans(x, y).rot_rad(rotation).trans(
                -25.0,
                -25.0,
            );

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
        self.draw_line(args);
        self.draw_obstacles(args);

        let mut flag = false;
        {
            let lobj = self.obstacles.last_mut();
            match lobj {
                Some(o) => {
                    if args.width as i32 - o.pos_x > 100 {
                        flag = true;
                    }
                }
                None => flag = true,
            }
        }
        if flag {
            let mut nobj = Object::new();
            nobj.pos_x = (args.width - nobj.width) as i32;
            nobj.pos_y = (args.height * 2 / 3 - nobj.height) as i32;
            self.obstacles.push(nobj);
        }
    }

    fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.rotation += 2.0 * args.dt;


    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("spinning-square", [600, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
