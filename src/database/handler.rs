use chrono::{NaiveDate, NaiveDateTime};
use duckdb::{params, Error};
use duckdb::ffi::duckdb_timestamp;
use crate::config::GLOBAL_PING_DATA;
use crate::database;
use crate::database::{PingData, POOL};
use crate::ping::result::PingResult;

pub fn insert_ping_data(data: PingResult) -> duckdb::Result<usize> {
    let pool_guard = POOL.lock().unwrap();
    let pool = pool_guard.as_ref().unwrap();

    let conn = pool.get().unwrap();
    conn.execute("INSERT INTO main.ping_data (host, latency, iface, timeout) VALUES (?, ?, ?, ?)", params![data.host, data.latency, data.iface, data.timeout])

}

pub fn select_ping_data_day(host:String, start:String) -> Result<Vec<PingData>, Error> {
    let pool_guard = POOL.lock().unwrap();
    let pool = pool_guard.as_ref().unwrap();

    let conn = pool.get().unwrap();
    let mut stmt = conn.prepare("SELECT
        DATE_TRUNC('day', CAST(created_at AS TIMESTAMP)) AS day,
        AVG(latency) AS avg_latency,
        MAX(latency) AS max_latency,
        MIN(latency) AS min_latency,
        SUM(CASE WHEN timeout THEN 1 ELSE 0 END) * 1.0 / COUNT(*) AS timeout_rate
    FROM
        ping_data
    WHERE
        created_at > CAST(? AS TIMESTAMP)
        and host = ?
    GROUP BY
        day
    ORDER BY
        day;")?;
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

    let rows = stmt.query_map([start,host], |row|{
        let day_timestamp: i64 = row.get(0)?; // Assuming the date is returned as a timestamp
        let day = NaiveDateTime::from_timestamp(day_timestamp*24*60*60, 0);
        Ok(
            PingData{
                host: "".to_string(),
                avg_latency: row.get(1)?,
                max_latency: row.get(2)?,
                min_latency: row.get(3)?,
                iface: "".to_string(),
                loss: row.get(4)?,
                time: Some(day.to_string()),
            }
        )
    })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }

    Ok(result)
}