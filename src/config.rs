use raylib::prelude::*;

pub struct Config {
    pub window_size: (i32, i32),
    pub bg_color: Color,
}

impl Config {
    pub fn new() -> Config {
        Config {
            window_size: (300, 300),
            bg_color: Color::new(30, 30, 46, 255),
        }
    }
}
