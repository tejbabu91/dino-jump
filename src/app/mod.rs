mod object;

use self::object::Object;
use graphics;
use opengl_graphics::GlGraphics;
use piston::input::*;
use rand;
use rand::Rng;

pub struct App {
    gl: GlGraphics,
    // OpenGL drawing backend.
    obstacles: Vec<Object>,
    px_move: f64,
    step_speed: f64,
    // px per sec
    window_width: u32,
    window_height: u32,
}

impl App {
    pub fn new(gl: GlGraphics) -> App {
        App {
            gl: gl,
            obstacles: vec![],
            px_move: 0.0,
            step_speed: 200.0,
            window_width: 0,
            window_height: 0,
        }
    }

    fn add_object(&mut self, o: Object) {
        self.obstacles.push(o)
    }

    fn draw_line(&mut self, args: &RenderArgs) {
        const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
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
        }

        if !self.obstacles.is_empty() && self.obstacles[0].pos_x < 0 {
            self.obstacles.remove(0);
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        const BACKGROUND_COLOR: [f32; 4] = [0.66, 0.66, 0.66, 1.0];
        self.window_width = args.width;
        self.window_height = args.height;
        self.gl.draw(args.viewport(), |_, gl| {
            // Clear the screen.
            graphics::clear(BACKGROUND_COLOR, gl);
        });
        self.draw_line(args);
        self.draw_obstacles(args);
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.px_move += self.step_speed * args.dt;
        let abs_px = self.px_move as i32;
        // println!("{}, {}", self.px_move, abs_px);
        if abs_px >= 1 {
            for obj in &mut self.obstacles {
                obj.move_by_px_x(-1 * abs_px);
            }
            self.px_move -= abs_px as f64;
            // println!("new px: {}", self.px_move);
        }
        let mut flag = false;
        if let Some(o) = self.obstacles.last_mut() {
            if self.window_width as i32 > o.pos_x {
                flag = true;
            }
        } else {
            flag = true;
        }

        if flag {
            let mut rng = rand::thread_rng();
            let mut nobj = Object::new();
            nobj.pos_x = (self.window_width - nobj.width + rng.gen_range(25, 300)) as i32;
            nobj.pos_y = (self.window_height * 2 / 3 - nobj.height) as i32;
            // println!("Adding object: {:?}", nobj);
            self.add_object(nobj);
        }
        self.step_speed += 0.001;
    }
}
