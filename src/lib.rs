// Modules
pub mod player;
pub mod stage;

// Exports
pub use player::Player;
pub use stage::Stage;

// Window configuration
use macroquad::prelude::Conf;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Platform".to_owned(),
        window_width: 1980,
        window_height: 1080,
        fullscreen: false,
        window_resizable: false,
        sample_count: 4,
        ..Default::default()
    }
}
