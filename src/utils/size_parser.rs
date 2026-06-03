use anyhow::{bail, Result};
use regex::Regex;

pub fn parse(size:&String) -> Result<u64> {
    let clean_input = size.replace(" ","").to_uppercase();

    let re = Regex::new(r"^(\d+(?:\.\d+)?)(B|KB|MB|GB|TB)?$")
        .expect("Regex compilation failed!");;

    if let Some(caps) = re.captures(&clean_input) {
        // Extract Number Group
        let num_str = caps.get(1).unwrap().as_str();
        let number:f64 = num_str.parse()?;

        // Extract the unit (Group 2) - default to "B" if missing
        let unit = caps.get(2).map_or("B", |b| b.as_str());

        // Determine multiplier
        let multiplier:u64 = match unit {
            "B" => 1,
            "KB" => 1_000,
            "MB" => 1_000_000,
            "GB" => 1_000_000_000,
            "TB" => 1_000_000_000_000,
            _ => bail!("Unknow size unit: {}",unit)
        };

        // Calculate total bytes
        let total_bytes = (number * multiplier as f64) as u64;

        Ok(total_bytes)

    }else {
        bail!("Size parsing error")
    }
}