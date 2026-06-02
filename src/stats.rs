use std::collections::HashMap;
use std::ffi::{OsStr, OsString};
use crate::models::FileInfo;

pub struct TotalFileAndDirsCount {
    pub total_files:u64,
    pub total_dirs:u64
}

pub fn total_file_and_dirs(files_info:&[FileInfo]) -> TotalFileAndDirsCount {
    let mut file_dir_count = TotalFileAndDirsCount {
        total_dirs:0,
        total_files:0
    };

    for file in files_info {
        if file.path.is_file() {
            // println!("[FILE 📄]:{}",file.path.display());
            file_dir_count.total_files +=1;
        }
        if file.path.is_dir() {
            // println!("[DIR 📂]:{}",file.path.display());
            file_dir_count.total_dirs+=1;
        }
    }
   file_dir_count
}


pub fn largets_files(files:&mut [FileInfo],top:usize) -> Vec<&FileInfo> {
    let mut largets_files:Vec<&FileInfo> = Vec::new();

    if files.is_empty() {
        eprintln!("Warn:(Largest Files) Files are empty can not calculate top largest files");
    }

    files.sort_by(|a,b| b.size.cmp(&a.size) );

    if let Some(file) = files.first() {
        println!("\n Largest File:{:?} \n\n",file);
    }

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



