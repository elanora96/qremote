use axum::{
    routing::{any, get},
    Router,
};
use gethostname::gethostname;
use local_ip_address::local_ip;
use qremote::{
    frontend::{build_template, serve_frontend},
    websockets::ws_handler,
    HostState,
};
use std::sync::Arc;
use tower_http::services::ServeDir;

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
