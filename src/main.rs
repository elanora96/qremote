use std::{ffi::OsString, net::IpAddr, sync::Arc};

use axum::{
    routing::{any, get},
    Router,
};
use gethostname::gethostname;
use local_ip_address::local_ip;
use qrcode::{render::unicode, QrCode};
use tower_http::services::ServeDir;

mod websockets;
use crate::websockets::ws_handler;

mod frontend;
use crate::frontend::{build_template, serve_frontend};

mod media_controls;

pub struct HostState {
    hostname: OsString,
    ip: IpAddr,
    port: u16,
}

impl HostState {
    fn get_remote_url(&self) -> String {
        format!("http://{}:{}/remote", self.ip, self.port)
    }

    fn get_qrcode_image(&self) -> String {
        let qrcode = QrCode::new(&self.get_remote_url()).unwrap();
        qrcode.render::<unicode::Dense1x2>().build()
    }

    fn print_host_ready(&self) {
        println!("{}{}", &self.get_remote_url(), &self.get_qrcode_image());
    }
}

#[tokio::main]
async fn main() {
    let handlebars = build_template();

    let host_state = Arc::new(HostState {
        hostname: gethostname(),
        ip: local_ip().unwrap(),
        port: 3030,
    });

    let app = Router::new()
        .route(
            "/remote",
            get({
                let hs = Arc::clone(&host_state);
                let hb = Arc::clone(&handlebars);
                move || serve_frontend(hs, hb)
            }),
        )
        .route("/ws", any(ws_handler))
        .nest_service("/assets", ServeDir::new("assets"));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", host_state.port))
        .await
        .unwrap();

    host_state.print_host_ready();

    axum::serve(listener, app).await.unwrap();
}
