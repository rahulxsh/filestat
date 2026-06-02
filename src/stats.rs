use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use crate::models::FileInfo;

pub fn largets_files(files:&mut [FileInfo],top:usize) -> Vec<&FileInfo> {
    let mut largets_files:Vec<&FileInfo> = Vec::new();

    if files.is_empty() {
        eprintln!("Warn:(Largest Files) Files are empty can not calculate top largest files");
    }

    files.sort_by(|a,b| b.size.cmp(&a.size) );

    for (index, file) in files.iter().take(top).enumerate() {
        largets_files.push(file)
    }
    largets_files
}


pub fn extension_count(files:&[FileInfo],extension:&str) -> HashMap<OsString,u64> {
    let mut extensions_count:HashMap<OsString,u64> = HashMap::new();

    for file in files {

        if file.path.is_file() {
            if let Some(ext) = file.path.extension() {
                *extensions_count.entry(ext.to_os_string()).or_insert(0) += 1;
                if ext == OsStr::new(extension) {
                    println!("{extension} File -> {}", file.path.display());
                }
            }
        }
    }
    extensions_count
}



