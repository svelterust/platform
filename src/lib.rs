// Modules
pub mod player;
pub mod stage;

// Exports
pub use player::Player;
pub use stage::Stage;

// Simple Result type
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

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

// Collideable
pub trait Collideable {
    fn bounding_box(&self) -> (f32, f32, f32, f32);
}

pub fn collides(this: &dyn Collideable, other: &dyn Collideable) -> bool {
    let (x1, y1, w1, h1) = this.bounding_box();
    let (x2, y2, w2, h2) = other.bounding_box();
    x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
}
