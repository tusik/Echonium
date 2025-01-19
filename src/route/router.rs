use axum::{
    response::Html,
    routing::get,
    Router,
};
use tower_http::services::ServeDir;
use askama::Template;
use crate::{config, template};

async fn index() -> Html<String> {
    let mut template = template::PingTemplate::new();
    let config = config::GLOBAL_PING_DATA.lock().await.clone();
    template.set_source(config);
    Html(template.render().unwrap())
}

pub mod api{
    use axum::extract::Path;
    use serde::Deserialize;
    use crate::config;
    use crate::database::handler::select_ping_data_day;
    #[derive(Deserialize, Debug)]
    pub(crate) struct PathParams {
        start: String,
        title: String,
    }

    pub(crate) async fn group_by_day(Path(param):Path<PathParams>) -> String {
        let mut data = config::GLOBAL_PING_DATA.lock().await;
        let mut host = "".to_string();
        println!("{:?}", param);
        for item in data.host.clone(){
            if item.title.is_some_and(|x| x == param.title){
                host = item.host;
                break;
            }
        }

        match select_ping_data_day(host,param.start){
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
        .route("/api/ping/day/:start/:title", get(api::group_by_day))
        .nest_service("/static", ServeDir::new("static"))
}