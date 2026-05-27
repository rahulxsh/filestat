use std::ffi::OsStr;
use std::fs::Permissions;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;

fn main() {
    let path = Path::new("./src");
    scan(path);
}

fn scan(path: &Path) {
    let mut total_file = 0;
    let mut total_dir = 0;
    let mut files_info:Vec<FileInfo> = Vec::new();

    for entry in WalkDir::new(path) {
        let entry = match entry {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        let file_type = entry.file_type();
        let metadata = entry.metadata();

        if file_type.is_file() {
            if let Ok(value) = metadata {
                let f_info = FileInfo {
                    path:entry.path().to_path_buf(),
                    size:value.len(),
                    created:value.created().ok(),
                    modified:value.modified().ok(),
                    permissions:value.permissions(),
                    accessed:value.accessed().ok()
                };

                files_info.push(f_info);
            }
        }

        if file_type.is_dir() {
            total_dir += 1;
            println!("[DIR] {}", entry.path().display());
        }

        if file_type.is_file() {
            total_file += 1;
            println!("[FILE] {}", entry.path().display());

            if let Some(ext) = entry.path().extension() {
                if ext == OsStr::new("rs") {
                    println!("Rust File -> {}", entry.path().display());
                }
            }
        }
    }

    println!("Total files: {}", total_file);
    println!("Total dirs: {}", total_dir);

    if !files_info.is_empty(){
        for i in files_info {
            println!("{:?} \n",i);
        }
    }
}

#[derive(Debug)]
pub struct FileInfo {
    pub path:PathBuf,
    pub size: u64,
    pub created:Option<SystemTime>,
    pub permissions: Permissions,
    pub accessed:Option<SystemTime>,
    pub modified:Option<SystemTime>
}