mod ping;
mod route;
mod schedule;
mod config;
mod template;
mod database;

use std::fmt::Error;
use std::time::Duration;
use axum::Router;
use surge_ping::{IcmpPacket, PingIdentifier, SurgeError};
use tokio::time::interval;
use crate::config::PingConfig;

async fn read_config() -> PingConfig {
    // 读取 TOML 文件内容
    let toml_content = tokio::fs::read_to_string("config.toml").await.expect("TODO: panic message");

    // 解析 TOML 内容为 Config 结构体
    let config: config::PingConfig = toml::from_str(&toml_content).expect("TODO: panic message");
    config
}

#[tokio::main]
async fn main() {
    {
        let mut data = config::GLOBAL_PING_DATA.lock().await;
        *data = read_config().await;
        let pool = database::create_pool(data.database_url.clone().unwrap_or("sqlite://memory".to_string())).await;
        database::init_pool(pool);
        for target in data.host.clone() {

            let _t = target.clone();
            let mut interval = interval(Duration::from_secs(_t.interval.unwrap_or(1)));
            tokio::spawn(async move {
                loop{
                    interval.tick().await;
                    let res = ping::icmp::ping(&_t).await.expect("TODO: panic message");
                    println!("ping result: {:?}", res);
                }

            });
        }
    }

    let app = route::router::create_router();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:12333").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[tokio::test]
async fn ping_test(){
    let payload = [0; 8];

    let res = ping::icmp::ping("127.0.0.1", 6, None).await.expect("TODO: panic message");

    println!("ping_test: {:?}", res);
    assert_eq!(res.0.get_identifier(), PingIdentifier(111));

}