use std::path::PathBuf;
use crate::fim::watch::models::{BaseLineFile};
use anyhow::Result;

pub fn build(path:&PathBuf, ignore_paths:Vec<String>) -> Result<BaseLineFile> {
    #[allow(unused)]
    let mut ig_path= None;

    if !ignore_paths.is_empty() {
        ig_path =  Some(ignore_paths);
    }else {
        ig_path = None
    }

    let baseline = BaseLineFile::build(path,ig_path)?;

    Ok(baseline)
}