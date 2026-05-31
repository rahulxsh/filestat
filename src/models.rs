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