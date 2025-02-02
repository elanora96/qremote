use super::ClientEventMessage;
use axum::{
    extract::{
        ws::{CloseFrame, Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
        ConnectInfo,
    },
    response::IntoResponse,
};
use axum_extra::{headers, TypedHeader};
use futures::{SinkExt, StreamExt};
use std::{net::SocketAddr, ops::ControlFlow};

/// The handler for the HTTP request (this gets called when the HTTP request lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };
    println!("`{user_agent}` at {addr} connected.");
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

/// Split the socket to send and recieve at the same time
/// Then handle sending and recieving
async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        #[allow(while_true)]
        while true {
            if sender
                .send(Message::Text(format!("I'm annoying").into()))
                .await
                .is_err()
            {
                println!("Sending close to {who}...");
                if let Err(e) = sender
                    .send(Message::Close(Some(CloseFrame {
                        code: axum::extract::ws::close_code::NORMAL,
                        reason: Utf8Bytes::from_static("Goodbye"),
                    })))
                    .await
                {
                    println!("Could not send Close due to {e}, probably it is ok?");
                }
                break;
            }

            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
        }
    });

    let recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            if process_message(msg, who).is_break() {
                break;
            }
        }
    });

    tokio::select! {
        _ = send_task => {
            println!("send_task completed")
        },
        _ = recv_task => {
            println!("recv_task completed")
        }
    }
}

/// Implementations for ws::Message
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => match serde_json::from_str::<ClientEventMessage>(&t) {
            Ok(event) => {
                event.execute();
            }
            Err(_) => {
                println!(">>> {who} sent str: {t:?}");
            }
        },
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }
        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }
        Message::Ping(v) => {
            println!(">>> {who} sent ping with {v:?}");
        }
    }
    ControlFlow::Continue(())
}
