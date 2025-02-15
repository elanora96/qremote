use super::key_map;
use super::HostState;
use axum::{
    http::{header, StatusCode, Uri},
    response::{IntoResponse, Response},
};
use handlebars::Handlebars;
use rust_embed::Embed;
use serde_json::json;
use std::str;
use std::sync::Arc;

#[derive(Embed)]
#[folder = "assets"]
struct Asset;

#[derive(Embed)]
#[folder = "templates"]
struct Template;

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

pub fn build_template() -> Arc<Handlebars<'static>> {
    let embedded_template = Template::get("index.hbs").unwrap().data.into_owned();

    Arc::new({
        let mut hbs = Handlebars::new();
        hbs.register_template_string("index", str::from_utf8(&embedded_template).unwrap())
            .unwrap();
        hbs
    })
}

pub async fn ui_handler(
    hs: Arc<HostState>,
    hb: Arc<Handlebars<'static>>,
) -> axum::response::Html<String> {
    let buttons: Vec<&String> = key_map().keys().collect();

    let data = json!({
        "title": "QRemote",
        "hostname": hs.hostname.to_string_lossy(),
        "ip": hs.ip,
        "port": hs.port,
        "buttons": buttons});

    let rendered = hb.render("index", &data).unwrap();

    axum::response::Html(rendered)
}
