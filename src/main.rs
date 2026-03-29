use raylib::prelude::*;

#[derive(Default)]
struct Player {
    pos: Vector2,
    speed: Vector2,
}

impl Player {
    fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) { self.speed.y = Player::JUMP; }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) { self.pos.x -= Player::RUN; }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) { self.pos.x += Player::RUN; }

        // Gravity
        self.pos.y -= self.speed.y;
        if self.speed.y > -10.0 {
            self.speed.y -= Player::GRAVITY;
        }
    }
    
    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle(self.pos.x as i32, self.pos.y as i32, Self::WIDTH, Self::HEIGHT, Color::BLACK);
    }

    // Constants
    const RUN: f32 = 5.0;
    const JUMP: f32 = 10.0;
    const GRAVITY: f32 = 0.3;
    const WIDTH: i32 = 64;
    const HEIGHT: i32 = 64;
}

#[derive(Default)]
struct World {
    player: Player,
}

impl World {
    fn update(&mut self, rl: &RaylibHandle) {
        self.player.update(rl);
    }
    
    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        self.player.draw(d);
    }
}

fn main() {
    // Setup
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Platform")
        .vsync()
        .build();

    // Game loop
    let mut world = World::default();
    while !rl.window_should_close() {
        world.update(&rl);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        world.draw(&mut d);
    }
}
