use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

pub static GLOBAL_PING_DATA: Lazy<Mutex<PingConfig>> = Lazy::new(|| Mutex::new(
    PingConfig {
        host: vec![],
        database_url: None,
    }
));
#[derive(Debug, Deserialize,Serialize,Clone)]
pub(crate) struct PingConfig {
    pub host:Vec<PingTarget>,
    pub database_url:Option<String>,
}
#[derive(Debug, Deserialize,Serialize,Clone)]
pub(crate) struct PingTarget{
    pub title:Option<String>,
    pub host:String,
    pub size:usize,
    pub iface:Option<String>,
    pub identifier:Option<u16>,
    pub timeout:Option<u64>,
    pub interval:Option<u64>,
}
