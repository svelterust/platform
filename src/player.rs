use crate::{Collideable, Stage, collides};
use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub h_speed: f32,
    pub v_speed: f32,
}

impl Collideable for Player {
    fn bounding_box(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.width, self.height)
    }
}

impl Player {
    pub fn new() -> Self {
        Player {
            x: screen_width() / 2.0 - 32.0,
            y: screen_height() / 2.0 - 32.0,
            width: 64.0,
            height: 64.0,
            h_speed: 0.0,
            v_speed: 0.0,
        }
    }

    pub fn update(&mut self, stage: &Stage) {
        // Inputs
        self.h_speed = 0.0;
        if is_key_down(KeyCode::Left) {
            self.h_speed = -5.0;
        }
        if is_key_down(KeyCode::Right) {
            self.h_speed = 5.0;
        }
        if is_key_down(KeyCode::Down) {
            self.v_speed += 1.0;
        }
        if is_key_down(KeyCode::Space) {
            self.v_speed = 7.0;
        }

        // Movement
        self.v_speed -= 0.2;
        self.pixel_move(self.h_speed, 0.0, stage);
        self.pixel_move(0.0, -self.v_speed, stage);
    }

    fn pixel_move(&mut self, dx: f32, dy: f32, stage: &Stage) {
        // Move horizontally
        for _ in 0..(dx.abs() as i32) {
            self.x += dx.signum();
            if self.check_collision(stage) {
                self.x -= dx.signum();
                break;
            }
        }

        // Move vertically
        for _ in 0..(dy.abs() as i32) {
            self.y += dy.signum();
            if self.check_collision(stage) {
                self.y -= dy.signum();
                if dy != 0.0 {
                    self.v_speed = 0.0;
                }
                break;
            }
        }
    }

    fn check_collision(&self, stage: &Stage) -> bool {
        let main_collision = collides(self, &stage.main);
        let platform_collision = stage
            .platforms
            .iter()
            .any(|platform| collides(self, platform));
        main_collision || platform_collision
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, WHITE);
    }
}
