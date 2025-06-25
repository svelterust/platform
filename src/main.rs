use macroquad::prelude::*;
use platform::*;

#[macroquad::main(window_conf)]
async fn main() {
    // State
    let stage = Stage::new();
    let mut player = Player::new();

    // Game logic
    loop {
        clear_background(LIGHTGRAY);
        player.update();
        player.draw();
        stage.draw();
        next_frame().await
    }
}
