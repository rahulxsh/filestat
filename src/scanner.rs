use std::fs::metadata;
use std::path::Path;
use walkdir::WalkDir;
use crate::models::{FileInfo, ScanResult};
use anyhow::{bail, Result};
use crate::metadata::get_metadata;

pub fn scan(path: &Path, hidden:bool) -> Result<ScanResult>{
    let mut scan_result = ScanResult {
        files:Vec::new(),
        total_dirs:0
    };

    if !path.exists() {
        bail!("Path does not exist: {}", path.display());
    }

    if !path.is_dir() {
        bail!("Path is not a directory: {}", path.display());
    }

    for entry_dir in WalkDir::new(path) {
        let entry_dir = match entry_dir {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Walk Error: {}", err);
                continue;
            }
        };

        if !hidden {
            if entry_dir.path().components().any(|c| {
                c.as_os_str()
                    .to_str()
                    .map(|s| s.starts_with(".") && s != ".")
                    .unwrap_or(false)
            }) {
                continue;
            }
        }

        let file_type = entry_dir.file_type();
        if file_type.is_dir() {
            scan_result.total_dirs +=1;
        }
        let is_file = file_type.is_file();
        let metadata = match get_metadata(&entry_dir) {
            Ok(m) => m,
            Err(e) => {
                eprintln!(
                    "Metadata error for {}: {}",
                    entry_dir.path().display(),
                    e
                );
                continue;
            }
        };
        
        if is_file {
            scan_result.files.push(metadata);
        }
    }

    Ok(scan_result)
}