use enigo::Key;
use lazy_static::lazy_static;
use qrcode::{render::unicode, QrCode};
use std::collections::HashMap;
use std::{ffi::OsString, net::IpAddr};

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

lazy_static! {
    pub static ref KEY_MAP: HashMap<String, Key> = [
        ("Stop".to_string(), Key::MediaStop),
        ("PlayPause".to_string(), Key::MediaPlayPause),
        ("NextTrack".to_string(), Key::MediaNextTrack),
    ]
    .iter()
    .cloned()
    .collect();
}
