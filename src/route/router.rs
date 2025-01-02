use axum::{
    response::Html,
    routing::get,
    Router,
};
use askama::Template;
use crate::{config, template};

async fn index() -> Html<String> {
    let mut template = template::PingTemplate::new();
    let config = config::GLOBAL_PING_DATA.lock().await.clone();
    template.set_source(config);
    Html(template.render().unwrap())
}

pub fn create_router() -> Router {
    Router::new().route("/ping", get(index))
}