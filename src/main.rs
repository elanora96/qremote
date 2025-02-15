use axum::{
    routing::{any, get},
    Router,
};
use gethostname::gethostname;
use local_ip_address::local_ip;
use qremote::{
    frontend::{build_template, static_handler, ui_handler},
    websockets::ws_handler,
    HostState,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // TODO: Hide behind debug flag
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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
                move || ui_handler(hs, hb)
            }),
        )
        .route("/ws", any(ws_handler))
        .route("/assets/{*path}", get(static_handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", host_state.port))
        .await
        .unwrap();

    host_state.print_host_ready();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
