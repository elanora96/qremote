use axum::{
    Router,
    routing::{any, get},
};
use clap::Parser;
use gethostname::gethostname;
use local_ip_address::local_ip;
use qremote::{
    HostState,
    frontend::{static_handler, ui_handler},
    websockets::ws_handler,
};
use std::{net::SocketAddr, sync::Arc};
use tower_http::trace::{DefaultMakeSpan, TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,

    /// Specify the port to listen on
    #[arg(short, long, default_value_t = 3030)]
    port: u16,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let host_state = Arc::new(HostState {
        hostname: gethostname(),
        ip: local_ip().unwrap(),
        port: args.port,
    });

    let mut app = Router::new()
        .route(
            "/remote",
            get({
                let hs = Arc::clone(&host_state);
                move || ui_handler(hs)
            }),
        )
        .route("/ws", any(ws_handler))
        .route("/assets/{*path}", get(static_handler));

    if args.debug {
        println!("Debug mode is on");
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
                }),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();

        app = app.layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::default().include_headers(true)),
        );
    }

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
