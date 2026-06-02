use std::collections::HashMap;
use std::ffi::OsString;
use crate::models::{FileInfo, FilesSize, ScanResult, ScanStats};

pub fn largets_files(files:&mut [FileInfo],top:usize) -> Vec<&FileInfo> {
    let mut largets_files:Vec<&FileInfo> = Vec::new();

    if files.is_empty() {
        eprintln!("Warn:(Largest Files) Files are empty can not calculate top largest files");
    }

    files.sort_by(|a,b| b.size.cmp(&a.size) );

    for (_index, file) in files.iter().take(top).enumerate() {
        largets_files.push(file)
    }
    largets_files
}


pub fn extension_count(files:&[FileInfo]) -> HashMap<OsString,u64> {
    let mut extensions_count:HashMap<OsString,u64> = HashMap::new();

    for file in files {
            if let Some(ext) = file.path.extension() {
                *extensions_count.entry(ext.to_os_string()).or_insert(0) += 1;
        }
    }
    extensions_count
}

pub fn file_size(files:&[FileInfo]) -> FilesSize {
    let total:u64 = files.iter().map(|f| f.size).sum();

    let average = if files.is_empty() {
        0.0
    }else {
        total as f64 / files.len() as f64
    };

    FilesSize {
        total,
        average
    }
}


pub fn generate_stats(scan_result: &ScanResult) -> ScanStats {
    let size = file_size(&scan_result.files);
    let extensions = extension_count(&scan_result.files);

    ScanStats {
        total_files:scan_result.files.len(),
        total_dirs:scan_result.total_dirs,
        total_size:size.total,
        average_size:size.average,
        extension_count:extensions
    }
}


