use crate::map::Map;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
}

impl Player {
    pub fn new(x: f32, y: f32, angle: f32) -> Self {
        Player { x, y, angle }
    }

    pub fn move_forward(&mut self, distance: f32, map: &Map) {
        let new_x = self.x + self.angle.cos() * distance;
        let new_y = self.y + self.angle.sin() * distance;
        if !map.is_wall(new_x as usize, new_y as usize) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn move_backward(&mut self, distance: f32, map: &Map) {
        let new_x = self.x - self.angle.cos() * distance;
        let new_y = self.y - self.angle.sin() * distance;
        if !map.is_wall(new_x as usize, new_y as usize) {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        self.angle += angle;
        self.angle %= 2.0 * std::f32::consts::PI;
    }
}