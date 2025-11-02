use super::HostState;
use super::key_map;
use axum::{
    http::{StatusCode, Uri, header},
    response::{IntoResponse, Response},
};
use maud::{DOCTYPE, Markup, html};
use rust_embed::Embed;
use std::sync::Arc;

#[derive(Embed)]
#[folder = "assets"]
struct Asset;

pub struct StaticFile<T>(pub T);

impl<T> IntoResponse for StaticFile<T>
where
    T: Into<String>,
{
    fn into_response(self) -> Response {
        let path = self.0.into();

        match Asset::get(path.as_str()) {
            Some(content) => {
                let mime = mime_guess::from_path(path).first_or_octet_stream();
                ([(header::CONTENT_TYPE, mime.as_ref())], content.data).into_response()
            }
            None => (StatusCode::NOT_FOUND, "404 Not Found").into_response(),
        }
    }
}

pub async fn static_handler(uri: Uri) -> impl IntoResponse {
    let path = uri.path().trim_start_matches("/assets/").to_string();

    StaticFile(path)
}

pub async fn ui_handler(hs: Arc<HostState>) -> Markup {
    let buttons: Vec<&String> = key_map().keys().collect();
    let title = "QRemote";
    html! {
        (DOCTYPE)
        head {
            meta charset="UTF-8";
            meta name="viewport" content="width=device-width, initial-scale=1.0";
            link rel="stylesheet" href="/assets/styles.css";
            script type="module" src="/assets/app.js" {}
            title {(title)}
        }
        body {
            main {
                form .media-controls #form {
                    @for b in buttons {
                        button .media-button data-value=(b) {}
                    }
                }
                div .branding {
                    p {
                        strong {
                            "QRemote"
                        }
                    }
                    p {
                        (hs.hostname.to_string_lossy()) " (" (hs.ip) ":" (hs.port) ")"
                    }
                }
            }
        }
    }
}
