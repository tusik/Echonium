use duckdb::{params, Error};
use crate::config::GLOBAL_PING_DATA;
use crate::database;
use crate::database::POOL;
use crate::ping::result::PingResult;

pub fn insert_ping_data(data: PingResult) -> duckdb::Result<usize> {
    let pool_guard = POOL.lock().unwrap();
    let pool = pool_guard.as_ref().unwrap();

    let conn = pool.get().unwrap();
    conn.execute("INSERT INTO main.ping_data (host, latency, iface, timeout) VALUES (?, ?, ?, ?)", params![data.host, data.latency, data.iface, data.timeout])

}