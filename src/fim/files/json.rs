use std::path::PathBuf;
use crate::fim::models::{ScanStats};
use anyhow::Result;
use std::fs::write;

pub fn json_stats(stats:&ScanStats) -> Result<String> {
    let json = serde_json::to_string_pretty(stats)?;

    Ok(json)
}

pub fn save_json(path:&PathBuf,stats:&ScanStats) -> Result<()> {
    write(path,json_stats(stats)?).expect("Unable to write json file");
    Ok(())
}