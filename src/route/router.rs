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

pub mod api{
    use crate::database::handler::select_ping_data_day;
    pub(crate) async fn group_by_day() -> String {
        match select_ping_data_day(){
            Ok(data) => {
                let json = serde_json::to_string(&data).unwrap();
                json
            }
            Err(e) => {
                println!("error: {:?}", e);
                "[]".to_string()
            }
        }


    }
}
pub fn create_router() -> Router {
    Router::new().route("/ping", get(index))
        .route("/api/ping/day", get(api::group_by_day))
}