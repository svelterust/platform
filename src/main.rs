use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Platform".to_owned(),
        window_width: 1980,
        window_height: 1080,
        fullscreen: true,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    loop {
        clear_background(LIGHTGRAY);

        // Main platform (bottom)
        draw_rectangle(
            screen_width() / 2.0 - 600.0,
            screen_height() - 300.0,
            1200.0,
            300.0,
            DARKBLUE,
        );

        // Left platform
        draw_rectangle(
            screen_width() / 2.0 - 750.0,
            screen_height() - 600.0,
            360.0,
            60.0,
            BLUE,
        );

        // Center platform
        draw_rectangle(
            screen_width() / 2.0 - 180.0,
            screen_height() - 750.0,
            360.0,
            60.0,
            BLUE,
        );

        // Right platform
        draw_rectangle(
            screen_width() / 2.0 + 390.0,
            screen_height() - 600.0,
            360.0,
            60.0,
            BLUE,
        );

        next_frame().await
    }
}
