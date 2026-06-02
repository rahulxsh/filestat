use std::collections::HashMap;
use std::ffi::OsString;
use std::fs::Permissions;
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Debug)]
#[allow(unused)]
pub struct FileInfo {
    pub path:PathBuf,
    pub size: u64,
    pub created:Option<SystemTime>,
    pub permissions: Permissions,
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


#[derive(Debug)]
pub struct ScanStats {
    pub total_files:usize,
    pub total_dirs:usize,
    pub total_size:u64,
    pub average_size:f64,
    pub extension_count:HashMap<OsString,u64>
}