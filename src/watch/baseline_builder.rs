use std::path::PathBuf;
use crate::watch::models::{BaseLineFile, Baseline};
use anyhow::Result;

pub fn build(path:&PathBuf) -> Result<BaseLineFile> {
    let baseline = BaseLineFile::build(path)?;

    Ok(baseline)
}