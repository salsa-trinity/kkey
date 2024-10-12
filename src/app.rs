use crate::config::Config;
use device_query::{DeviceQuery, DeviceState, Keycode};
use raylib::prelude::*;

pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
    state: DeviceState,

    pub is_open: bool,
}

impl App {
    pub fn new() -> App {
        let config = Config::new();
        let (rl, thread) = raylib::init()
            .size(config.window_size.0, config.window_size.1)
            .undecorated()
            .build();
        App {
            rl,
            thread,
            is_open: true,
            state: DeviceState::new(),
        }
    }

    pub fn init(&mut self) {
        self.rl.set_target_fps(60);
    }

    pub fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        let config = Config::new();
        //d.draw_fps(5, 5);
        d.clear_background(config.bg_color);

        let keys = self.state.get_keys();
        for key in keys {
            for skey in &config.f_keys {
                if key == skey.keycode {
                    d.draw_text(&skey.keycode.to_string(), 0, 0, 26, Color::GREEN);
                }
            }
        }
    }
}
