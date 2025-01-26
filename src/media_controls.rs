use super::KEY_MAP;
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

pub fn str_to_keypress(input: &str) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    let mut keypress = |key: Key| enigo.key(key, Click);

    let matched_key = KEY_MAP.get(input).unwrap().clone();

    if let Err(e) = keypress(matched_key) {
        println!("{e}")
    }
}
