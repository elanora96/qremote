use super::key_map;
use enigo::{Direction::Click, Enigo, Key, Keyboard, Settings};

pub fn str_to_key(input: &str) -> Result<Key, &str> {
    match key_map().get(input) {
        Some(k) => Ok(k.to_owned()),
        None => Err("Invalid key {input}"),
    }
}

pub fn str_to_keypress(input: &str) {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();

    match str_to_key(input) {
        Ok(k) => match enigo.key(k, Click) {
            Ok(_) => (),
            Err(e) => println!("{e:?}"),
        },
        Err(e) => {
            println!("{e:?}")
        }
    }
}
