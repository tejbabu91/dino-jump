#[derive(Debug)]
pub struct Object {
    pub width: u32,
    pub height: u32,
    pub pos_x: i32,
    pub pos_y: i32,
}

impl Object {
    pub fn move_by_px_x(&mut self, offset: i32) {
        self.pos_x += offset;
    }

    pub fn new() -> Object {
        Object {
            width: 10,
            height: 20,
            pos_x: 0,
            pos_y: 0,
        }
    }

    pub fn get_draw_coordinates(&self) -> (f64, f64, f64, f64) {
        (
            self.pos_x as f64,
            self.pos_y as f64,
            (self.pos_x + self.width as i32) as f64,
            (self.pos_y + self.height as i32) as f64,
        )
    }
}
