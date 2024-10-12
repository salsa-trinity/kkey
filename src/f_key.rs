use device_query::Keycode;

pub struct FKey {
    pub keycode: Keycode,
}

impl FKey {
    pub fn new(keycode: Keycode) -> FKey {
        FKey { keycode }
    }
}
