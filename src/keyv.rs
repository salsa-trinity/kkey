use device_query::Keycode;

pub struct Keyv {
    pub keycode: Keycode,
    pub visible_string: String,
    pub pos: (i32, i32),
    pub size: (i32, i32),
}

impl Keyv {
    pub fn new(
        keycode: Keycode,
        visible_string: String,
        pos: (i32, i32),
        size: (i32, i32),
    ) -> Keyv {
        Keyv {
            keycode,
            visible_string,
            pos,
            size,
        }
    }
}
