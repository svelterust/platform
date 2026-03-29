use raylib::prelude::*;

#[derive(Default)]
struct Player {
    x: i32,
    y: i32,
}

impl Player {
    fn update(&mut self, rl: &RaylibHandle) {
        if rl.is_key_down(KeyboardKey::KEY_UP) { self.y -= Player::SPEED; }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) { self.y += Player::SPEED; }
        if rl.is_key_down(KeyboardKey::KEY_LEFT) { self.x -= Player::SPEED; }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) { self.x += Player::SPEED; }
    }
    
    fn draw(&self, d: &mut RaylibDrawHandle<'_>) {
        d.draw_rectangle(self.x, self.y, 64, 64, Color::BLACK);        
    }

    // Constants
    const SPEED: i32 = 3;
}

fn main() {
    // Setup
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Platform")
        .vsync()
        .build();
    let mut player = Player::default();

    // Game loop
    while !rl.window_should_close() {
        player.update(&rl);
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);
        player.draw(&mut d);
    }
}
