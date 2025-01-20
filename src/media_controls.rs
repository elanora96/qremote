use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};
use std::collections::HashMap;

struct AllowedKey {
    key_name: String,
    key_code: Key,
}

struct AllowedKeys {
    keys: Vec<AllowedKey>,
}

impl AllowedKeys {
    fn get_key_by_name(&self, key_name: &str) {}
}

pub fn idk() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let allowed = AllowedKeys {
        keys: vec![AllowedKey {
            key_name: String::from("Next"),
            key_code: Key::MediaNextTrack,
        }],
    };

    let mut keypress = |key| enigo.key(key, Click);

    if let Err(e) = keypress(Key::MediaNextTrack) {
        println!("{e}")
    }
}
