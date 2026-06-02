use std::path::Path;
use walkdir::WalkDir;
use crate::models::FileInfo;
use anyhow::{bail, Result};

pub fn scan(path: &Path, hidden:bool) -> Result<Vec<FileInfo>>{
    let mut files_info:Vec<FileInfo> = Vec::new();

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
                eprintln!("Error: {}", err);
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
        let is_file = file_type.is_file();
        let metadata = match entry_dir.metadata() {
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
                let f_info = FileInfo {
                    path:entry_dir.path().to_path_buf(),
                    size:metadata.len(),
                    created:metadata.created().ok(),
                    modified:metadata.modified().ok(),
                    permissions:metadata.permissions(),
                    accessed:metadata.accessed().ok()
                };

                files_info.push(f_info);
        }
    }

    Ok(files_info)
}