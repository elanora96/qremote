use enigo::Key;
use qrcode::{render::unicode, QrCode};
use serde::Deserialize;
use std::{collections::HashMap, ffi::OsString, net::IpAddr, sync::OnceLock};

pub mod frontend;
pub mod media_controls;
pub mod websockets;

pub struct HostState {
    pub hostname: OsString,
    pub ip: IpAddr,
    pub port: u16,
}

impl HostState {
    pub fn get_remote_url(&self) -> String {
        format!("http://{}:{}/remote", self.ip, self.port)
    }

    pub fn get_qrcode_image(&self) -> String {
        let qrcode = QrCode::new(&self.get_remote_url()).unwrap();
        qrcode.render::<unicode::Dense1x2>().build()
    }

    pub fn print_host_ready(&self) {
        println!("{}{}", &self.get_remote_url(), &self.get_qrcode_image());
    }
}

#[derive(Debug, Deserialize)]
pub struct ClientEventMessage {
    #[serde(rename = "eventType")]
    event_type: String,
    #[serde(rename = "clickedKey")]
    clicked_key: Option<String>,
    // modifiers: Option<Vec<String>>, // TODO
}

impl ClientEventMessage {
    pub fn execute(&self) {
        match self.event_type.as_str() {
            "clicked" => {
                let ck = self.clicked_key.clone().unwrap();
                media_controls::str_to_keypress(&ck);
            }
            _ => println!("Unrecognized event_type!"),
        }
    }
}

pub fn key_map() -> &'static HashMap<String, Key> {
    static KEY_MAP: OnceLock<HashMap<String, Key>> = OnceLock::new();
    KEY_MAP.get_or_init(|| {
        let mut m = HashMap::new();
        m.insert("Stop".to_string(), Key::MediaStop);
        m.insert("PlayPause".to_string(), Key::MediaPlayPause);
        m.insert("NextTrack".to_string(), Key::MediaNextTrack);
        m
    })
}
