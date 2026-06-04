use std::collections::HashMap;
use std::path::PathBuf;
use crate::models::{ScanResult};
use anyhow::Result;
use blake3::Hash;
use crate::hashing::hash_file::hash_file;

pub fn get_duplicates(files:&ScanResult) -> Result<HashMap<Hash,Vec<PathBuf>>> {
    let mut map:HashMap<Hash,Vec<PathBuf>> = HashMap::new();

    for file in &files.files {
        let hash = hash_file(&file)?;
        map.entry(hash).or_default().push(file.path.clone())
    }

    Ok(map)
}