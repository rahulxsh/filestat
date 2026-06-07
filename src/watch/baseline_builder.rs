use std::path::PathBuf;
use crate::watch::models::Baseline;
use anyhow::Result;

pub fn build(path:&PathBuf) -> Result<Baseline> {
    let baseline = Baseline::build(path)?;

    Ok(baseline)
}