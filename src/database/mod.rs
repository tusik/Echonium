use std::sync::Mutex;
use once_cell::sync::Lazy;
use sqlx::sqlite::SqlitePool;
pub mod handler;

static POOL: Lazy<Mutex<SqlitePool>> = Lazy::new(|| {
    panic!("Database pool not initialized!");
});

pub fn init_pool(pool: SqlitePool) {
    let mut p = POOL.lock().unwrap();
    *p = pool;
}

pub fn create_pool(url: String) -> SqlitePool {
    let pool = SqlitePool::connect(url.as_str()).expect("TODO: panic message");
    pool
}