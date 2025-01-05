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
use crate::database::handler::insert_ping_data;

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
        database::create_pool(data.database_url.clone().unwrap_or("echonium.db".to_string()));
        if !database::check_table_exist("ping_data"){
            println!("table not exist, init table");
            database::init_table();
        }

        for target in data.host.clone() {

            let _t = target.clone();
            let mut interval = interval(Duration::from_secs(_t.interval.unwrap_or(1)));
            tokio::spawn(async move {
                loop{
                    interval.tick().await;
                    let res = ping::icmp::ping(&_t).await;
                    let mut ping_data = ping::result::PingResult::new();
                    ping_data.set_host(_t.host.clone());
                    ping_data.set_iface(_t.iface.clone().unwrap_or("".to_string()));

                    match res {
                        Ok(data) => {
                            ping_data.set_latency(data.1.as_secs_f64() * 1000.0);
                        }
                        Err(err) => {
                            match err {
                                SurgeError::IncorrectBufferSize => {}
                                SurgeError::MalformedPacket(_) => {}
                                SurgeError::IOError(_) => {}
                                SurgeError::Timeout { .. } => {
                                    ping_data.set_timeout(true);
                                }
                                SurgeError::EchoRequestPacket => {}
                                SurgeError::NetworkError => {}
                                SurgeError::IdenticalRequests { .. } => {}
                            }
                        }
                    }
                    insert_ping_data(ping_data).expect("TODO: panic message");
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