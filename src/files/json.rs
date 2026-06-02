use crate::models::ScanStats;
use anyhow::Result;

pub fn json_export(stats:&ScanStats)-> Result<String> {
    let json = serde_json::to_string_pretty(stats)?;

    Ok(json)
}