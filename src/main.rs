mod clap_config;
use clap::Parser;

use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use std::fs::Permissions;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;
use crate::clap_config::{Cli, Commands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, top, hidden } => {
            println!("Path: {:?}", path);
            println!("Top files: {}", top);
            println!("Include hidden: {}", hidden);

            let path = Path::new(&path);
            scan(path,top);
        }
    }
}

fn scan(path: &Path,top:usize) {
    let mut total_file = 0;
    let mut total_dir = 0;
    let mut files_info:Vec<FileInfo> = Vec::new();
    let mut extensions_count:HashMap<OsString,u64> = HashMap::new();

    for entry_dir in WalkDir::new(path) {
        let entry_dir = match entry_dir {
            Ok(e) => e,
            Err(err) => {
                eprintln!("Error: {}", err);
                continue;
            }
        };

        let file_type = entry_dir.file_type();
        let metadata = entry_dir.metadata();

        if file_type.is_file() {
            if let Ok(value) = metadata {
                let f_info = FileInfo {
                    path:entry_dir.path().to_path_buf(),
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
            println!("[DIR] {}", entry_dir.path().display());
        }

        if file_type.is_file() {
            total_file += 1;
            println!("[FILE] {}", entry_dir.path().display());

            if let Some(ext) = entry_dir.path().extension() {
                *extensions_count.entry(ext.to_os_string()).or_insert(0) += 1;
                if ext == OsStr::new("rs") {
                    println!("Rust File -> {}", entry_dir.path().display());
                }
            }
        }
    }

    println!("Total files: {}", total_file);
    println!("Total dirs: {}", total_dir);

    if !files_info.is_empty(){
        files_info.sort_by(|a,b| b.size.cmp(&a.size) );

        if let Some(file) = files_info.first() {
            println!("\n Largest File:{:?} \n\n",file);
        }

        for (index, file) in files_info.iter().take(top).enumerate() {
            println!(
                "{}. {} -> {} bytes",
                index + 1,
                file.path.display(),
                file.size
            );
        }

    }


    for (exten,count) in extensions_count {
        println!("{:?} -> {}",exten,count);
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