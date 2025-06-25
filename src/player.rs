use macroquad::prelude::*;

pub struct Player {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: screen_width() / 2.0 - 32.0,
            y: screen_height() / 2.0 - 32.0,
            width: 64.0,
            height: 64.0,
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Left) {
            self.x -= 3.0;
        }
        if is_key_down(KeyCode::Right) {
            self.x += 3.0;
        }
        if is_key_down(KeyCode::Up) {
            self.y -= 3.0;
        }
        if is_key_down(KeyCode::Down) {
            self.y += 3.0;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
    }
}
