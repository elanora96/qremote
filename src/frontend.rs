use handlebars::Handlebars;
use serde_json::json;
use std::sync::Arc;

use crate::HostState;

pub fn build_template() -> Arc<Handlebars<'static>> {
    Arc::new({
        let mut hbs = Handlebars::new();
        hbs.register_template_file("index", "templates/index.hbs")
            .unwrap();
        hbs
    })
}

pub async fn serve_frontend(
    hs: Arc<HostState>,
    hb: Arc<Handlebars<'static>>,
) -> axum::response::Html<String> {
    let buttons = ["volume_up", "volume_down", "pause_play", "stop"];

    let data = json!({
        "title": "QRemote",
        "hostname": hs.hostname.to_string_lossy(),
        "ip": hs.ip,
        "port": hs.port,
        "buttons": buttons});

    let rendered = hb.render("index", &data).unwrap();

    axum::response::Html(rendered)
}
