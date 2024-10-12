use crate::config::Config;
use raylib::prelude::*;

pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
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
        }
    }

    pub fn init(&mut self) {
        self.rl.set_target_fps(60);
    }

    pub fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        let config = Config::new();
        d.draw_fps(5, 5);
        d.clear_background(config.bg_color);
    }
}
