use std::path::PathBuf;
use std::time::{UNIX_EPOCH};
use rusqlite::{Connection, Result};
use dirs;
use crate::fim::watch::models::Alert;

pub fn get_db_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Could not determine home directory");

    path.push(".filestat");

    std::fs::create_dir_all(&path).unwrap();

    path.push("filestat.db");

    path
}
pub fn init_db(conn:&Connection) -> Result<()> {
    let alert_table_create = "CREATE TABLE IF NOT EXISTS alerts (
    id INTEGER PRIMARY KEY,
    timestamp INTEGER NOT NULL,
    severity TEXT NOT NULL,
    alert_type TEXT NOT NULL,
    path TEXT NOT NULL,
    old_hash TEXT,
    new_hash TEXT,
    old_size INTEGER,
    new_size INTEGER
);";

    conn.execute(alert_table_create,[])?;

    Ok(())
}


pub fn insert_alert(
    conn: &Connection,
    alert: &Alert,
) -> Result<()> {
    let time = alert.timestamp.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let old_size = &alert.old_size.map(|s| s as i64);
    let new_size = &alert.new_size.map(|s| s as i64);
    conn.execute(
        "
        INSERT INTO alerts (
            timestamp,
            severity,
            alert_type,
            path,
            old_hash,
            new_hash,
            old_size,
            new_size
        )
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
        ",
        (
            time,
            &alert.severity.to_string(),
            &alert.alert_type.to_string(),
            &alert.path.to_string_lossy(),
            &alert.old_hash,
            &alert.new_hash,
            &alert.old_size.map(|s| s as i64),
            &alert.new_size.map(|s| s as i64),
        ),
    )?;

    Ok(())
}