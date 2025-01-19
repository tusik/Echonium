use std::sync::{Arc, Mutex};
use duckdb::{params, DuckdbConnectionManager};
use once_cell::sync::Lazy;
use r2d2::Pool;
use chrono::{NaiveDate, NaiveDateTime};
use duckdb::arrow::datatypes::Date32Type;
use duckdb::ffi::{duckdb_time, duckdb_timestamp};
use serde::{Deserialize, Serialize};

pub mod handler;

static POOL: Lazy<Arc<Mutex<Option<Pool<DuckdbConnectionManager>>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(None))
});
#[derive(Debug,Deserialize,Serialize)]
pub struct PingData{
    host: String,
    avg_latency: f64,
    max_latency: f64,
    min_latency: f64,
    iface: String,
    loss: f64,
    time: Option<String>
}
pub fn create_pool(url: String) -> bool {
    let mgr = DuckdbConnectionManager::file(url).unwrap();

    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(mgr)
        .expect("Failed to create pool");

    // 使用 Mutex 的 lock 方法获取可变引用，并修改 POOL
    let mut pool_guard = POOL.lock().unwrap();
    *pool_guard = Some(pool);

    true
}

// 检查是否存在某张数据表
pub fn check_table_exist(table_name: &str) -> bool {
    let pool_guard = POOL.lock().unwrap();
    let pool = pool_guard.as_ref().unwrap();

    let conn = pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT count(*) FROM information_schema.tables WHERE table_name = ?").unwrap();
    let mut rows = stmt.query_map(&[&table_name], |row|{
        Ok(row.get::<_, i32>(0)?)
    }).unwrap();
    let count = rows.next().unwrap().unwrap();
    count > 0
}

pub fn init_table() {
    let pool_guard = POOL.lock().unwrap();
    let pool = pool_guard.as_ref().unwrap();

    let conn = pool.get().unwrap();
    conn.execute("CREATE SEQUENCE IF NOT EXISTS ping_id_seq START 1;", params![])
        .unwrap();
    conn.execute("\
        CREATE TABLE ping_data(id INTEGER DEFAULT(nextval('ping_id_seq')),
        host VARCHAR,
        latency DOUBLE,
        iface VARCHAR,
        timeout BOOLEAN,
        created_at TIMESTAMP DEFAULT(CURRENT_TIMESTAMP));",
                 [])
        .unwrap();
    
}