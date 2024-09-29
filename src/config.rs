use crate::keyv::Keyv;
use device_query::Keycode;
use raylib::color::Color;

pub struct Config {
    pub size: (i32, i32),
    pub max_fps: u32,
    pub bg: Color,
    pub key_pressed_bg: Color,
    pub key_not_pressed_bg: Color,
    pub keys: Vec<Keyv>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            size: (300, 300),
            max_fps: 60,
            bg: Color::new(30, 30, 46, 255),
            key_pressed_bg: Color::new(203, 166, 247, 255),
            key_not_pressed_bg: Color::new(180, 190, 254, 255),
            keys: vec![
                Keyv::new(Keycode::A, "A".to_string(), (50, 50), (50, 50)),
                Keyv::new(Keycode::B, "B".to_string(), (105, 105), (50, 50)),
            ],
        }
    }
}
