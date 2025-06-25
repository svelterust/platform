use macroquad::prelude::*;

struct Block {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Block {
    fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, BLUE);
    }
}

pub struct Stage {
    stage: Block,
    platforms: [Block; 3],
}

impl Stage {
    pub fn new() -> Self {
        Self {
            stage: Block::new(
                screen_width() / 2.0 - 600.0,
                screen_height() - 300.0,
                1200.0,
                300.0,
            ),
            platforms: [
                Block::new(
                    screen_width() / 2.0 - 750.0,
                    screen_height() - 600.0,
                    360.0,
                    60.0,
                ),
                Block::new(
                    screen_width() / 2.0 - 180.0,
                    screen_height() - 750.0,
                    360.0,
                    60.0,
                ),
                Block::new(
                    screen_width() / 2.0 + 390.0,
                    screen_height() - 600.0,
                    360.0,
                    60.0,
                ),
            ],
        }
    }

    pub fn draw(&self) {
        // Draw stage and platforms
        self.stage.draw();
        for platform in &self.platforms {
            platform.draw();
        }
    }
}
