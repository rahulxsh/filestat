use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;
use serde::Serialize;

#[derive(Debug,Serialize)]
#[allow(unused)]
pub struct FileInfo {
    pub path:PathBuf,
    pub size: u64,
    pub created:Option<SystemTime>,
    pub readonly: bool,
    pub accessed:Option<SystemTime>,
    pub modified:Option<SystemTime>
}

#[derive(Debug)]
pub struct ScanResult {
    pub files:Vec<FileInfo>,
    pub total_dirs:usize
}

#[derive(Debug)]
pub struct FilesSize {
    pub total:u64,
    pub average:f64
}


#[derive(Debug,Serialize)]
pub struct ScanStats<'a> {
    pub total_files:usize,
    pub total_dirs:usize,
    pub total_size:u64,
    pub average_size:f64,
    pub extension_count:HashMap<String,u64>,
    pub largest_files:Vec< &'a FileInfo>
}

#[derive(Debug,Serialize)]
pub struct ExtensionCount<'a> {
    pub extension:&'a String,
    pub count:u64
}

#[derive(Debug,Serialize)]
pub struct CsvScanStats<T> {
    pub key:String,
    pub value:T
}


#[derive(Debug,Serialize)]
pub struct CsvLargeFiles {
    pub path:PathBuf,
    pub size: u64,
}