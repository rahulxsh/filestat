use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::time::{Duration, UNIX_EPOCH};

use crate::fim::watch::models::{Alert, AlertType, Severity};

pub fn alerts(conn: &Connection, limit: i64) -> Result<Vec<Alert>> {
    let mut stmt = conn.prepare(
        "
        SELECT
            timestamp,
            severity,
            alert_type,
            path,
            old_hash,
            new_hash,
            old_size,
            new_size
        FROM alerts
        ORDER BY timestamp DESC
        LIMIT ?
        ",
    )?;

    let rows = stmt.query_map([limit], |row| {
        let timestamp: i64 = row.get(0)?;
        let severity: String = row.get(1)?;
        let alert_type: String = row.get(2)?;
        let path: String = row.get(3)?;

        Ok(Alert {
            timestamp: UNIX_EPOCH
                + Duration::from_secs(timestamp as u64),

            severity: Severity::from_str(&severity)
                .expect("invalid severity"),

            alert_type: AlertType::from_str(&alert_type)
                .expect("invalid alert type"),

            path: PathBuf::from(path),

            old_hash: row.get(4)?,
            new_hash: row.get(5)?,

            old_size: row
                .get::<_, Option<i64>>(6)?
                .map(|v| v as u64),

            new_size: row
                .get::<_, Option<i64>>(7)?
                .map(|v| v as u64),
        })
    })?;

    let mut alerts = Vec::new();

    for alert in rows {
        alerts.push(alert?);
    }

    Ok(alerts)
}


pub fn display_alerts(alert:Alert) {
    println!("\n");
    println!("Severity:[{}]",alert.severity);
    println!("PATH:{}",alert.path.display());

    if let Some(new_h) = alert.new_hash {
        println!("New Hash:{}",new_h);
    }
    if let Some(old_h) = alert.old_hash {
        println!("Old Hash:{}",old_h);
    }

    if let Some(new_s) = alert.new_size {
        println!("New Size:{}",new_s);
    }

    if let Some(old_s) = alert.old_size {
        println!("Old Size:{}",old_s);
    }
}