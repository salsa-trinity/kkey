use crate::config::Config;
use device_query::{DeviceQuery, DeviceState};
use raylib::prelude::*;

pub struct App {
    rl: RaylibHandle,
    thread: RaylibThread,
    config: Config,
    device: DeviceState,

    pub is_open: bool,
}

impl App {
    pub fn new() -> App {
        let (mut rl, thread) = raylib::init()
            .size(Config::new().size.0, Config::new().size.1)
            .undecorated()
            .build();
        rl.set_target_fps(Config::new().max_fps);
        App {
            rl,
            thread,
            config: Config::new(),
            is_open: true,
            device: DeviceState::new(),
        }
    }

    pub fn draw(&mut self) {
        let mut d = self.rl.begin_drawing(&self.thread);
        d.draw_fps(0, 0);
        d.clear_background(self.config.bg);

        let keys = self.device.get_keys();
        for key in keys {
            for keyv in &self.config.keys {
                if key == keyv.keycode {
                    print!("{}", keyv.visible_string);
                    d.draw_rectangle(
                        keyv.pos.0,
                        keyv.pos.1,
                        keyv.size.0,
                        keyv.size.1,
                        self.config.key_pressed_bg,
                    );
                }
            }
        }
    }
}
