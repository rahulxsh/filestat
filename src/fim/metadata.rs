use walkdir::DirEntry;
use anyhow::Result;
use crate::fim::models::FileInfo;

pub fn get_metadata(file:&DirEntry) -> Result<FileInfo> {
    let metadata = file.metadata()?;

    let f_info = FileInfo {
        path:file.path().to_path_buf(),
        size:metadata.len(),
        created:metadata.created().ok(),
        modified:metadata.modified().ok(),
        readonly:metadata.permissions().readonly(),
        accessed:metadata.accessed().ok()
    };

    Ok(f_info)
}