use std::path::{PathBuf};
use std::time::SystemTime;
use crate::fim::watch::models::{Alert, AlertType, Severity};

#[allow(unused)]
pub fn hash_changed_alert<'a>(
    path:PathBuf,
    old_hash:String,
    new_hash:String,
    old_size:u64,
    new_size:u64
) -> Alert {
    Alert {
        timestamp: SystemTime::now(),
        severity: Severity::High,
        alert_type: AlertType::HashChanged,
        path,

        old_hash: Some(old_hash),
        new_hash: Some(new_hash),

        old_size: Some(old_size),
        new_size: Some(new_size),
    }
}

pub fn print_alert(alert: &Alert) {
    println!();
    println!("[{}]", alert.severity);
    println!("{}", alert.alert_type);
    println!("Path: {}", alert.path.display());

    if let Some(old_hash) = &alert.old_hash {
        println!("Old Hash: {}", old_hash);
    }

    if let Some(new_hash) = &alert.new_hash {
        println!("New Hash: {}", new_hash);
    }

    if let Some(old_size) = alert.old_size {
        println!("Old Size: {}", old_size);
    }

    if let Some(new_size) = alert.new_size {
        println!("New Size: {}", new_size);
    }

    println!();
}