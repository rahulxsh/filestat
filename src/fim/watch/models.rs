use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::fim::models::FilterConfig;
use crate::fim::scanner::scan;
use anyhow::Result;
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use serde::{Deserialize, Serialize};
use crate::fim::hashing::hash_file::hash_file;

#[allow(unused)]
pub enum EventTypes{
    CREATE,
    MODIFY,
    REMOVE
}

#[derive(Debug,Serialize,Deserialize)]
#[allow(unused)]
pub struct Baseline {
    pub hashes:HashMap<PathBuf,String>
}

impl BaseLineFile {
    pub fn build(path:&PathBuf,ignore_paths:Option<Vec<String>>) -> Result<Self> {
        let filter_config = FilterConfig {
            ignore:ignore_paths,
            ext:None,
            min_size:None,
            max_size:None
        };
        let scanned_files = scan(path,true,filter_config)?;
        let mut map:HashMap<PathBuf,BaselineFileInfo> = HashMap::new();

        for file in scanned_files.files {
            let hash = hash_file(&file.path)?;
            let metadata = std::fs::metadata(&file.path)?;

            let size = metadata.len();
            let modified_time = metadata
                .modified()?
                .duration_since(UNIX_EPOCH)?
                .as_secs();

            let canonical = file.path.canonicalize()?;
            map.insert(canonical,BaselineFileInfo {
                size,
                hash:hash.to_string(),
                modified:modified_time
            });
        }

        Ok(Self {
            hashes:map
        })
    }

        pub fn get(&self,path:&Path) -> Option<&BaselineFileInfo> {
        let hash = self.hashes.get(path);
        hash
    }

    pub fn insert(&mut self, path:PathBuf, file_info:BaselineFileInfo) {
        self.hashes.insert(path,file_info);
    }

    pub fn remove(&mut self, path:&Path) -> Option<BaselineFileInfo> {
        let entry = self.hashes.remove(path);
        entry
    }
}

#[derive(Debug,Serialize,Deserialize)]
pub struct BaseLineFile {
    pub hashes:HashMap<PathBuf,BaselineFileInfo>
}
#[derive(Debug,Serialize,Deserialize)]
pub struct BaselineFileInfo {
    pub size:u64,
    pub hash:String,
    pub modified:u64
}

#[derive(Debug)]
#[allow(unused)]
pub enum AlertType {
    FileCreated,
    FileDeleted,

    DirectoryCreated,
    DirectoryDeleted,

    HashChanged,

    PermissionChanged,
    OwnershipChanged,
}

#[derive(Debug,Clone)]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Alert {
    pub timestamp: SystemTime,
    pub alert_type: AlertType,
    pub severity: Severity,
    pub path: PathBuf,

    pub old_hash: Option<String>,
    pub new_hash: Option<String>,

    pub old_size:Option<u64>,
    pub new_size:Option<u64>
}

impl Alert {
    pub fn new(self) -> Alert {
        let alert = Alert {
            timestamp:self.timestamp,
            alert_type:self.alert_type,
            severity:self.severity,
            path:self.path,
            old_hash:self.old_hash,
            new_hash:self.new_hash,
            old_size:self.old_size,
            new_size:self.new_size
        };

        alert
    }
}


impl Severity {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "LOW" => Some(Self::Low),
            "MEDIUM" => Some(Self::Medium),
            "HIGH" => Some(Self::High),
            "CRITICAL" => Some(Self::Critical),
            _ => None,
        }
    }
}

impl AlertType {
    pub fn from_str(value: &str) -> Option<Self> {
        match value {
            "FILE CREATED" => Some(Self::FileCreated),
            "FILE DELETED" => Some(Self::FileDeleted),
            "DIRECTORY CREATED" => Some(Self::DirectoryCreated),
            "DIRECTORY DELETED" => Some(Self::DirectoryDeleted),
            "HASH CHANGED" => Some(Self::HashChanged),
            "PERMISSION CHANGED" => Some(Self::PermissionChanged),
            "OWNERSHIP CHANGED" => Some(Self::OwnershipChanged),
            _ => None,
        }
    }
}
impl Display for Severity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Severity::Low => write!(f, "LOW"),
            Severity::Medium => write!(f, "MEDIUM"),
            Severity::High => write!(f, "HIGH"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Display for AlertType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AlertType::FileCreated => write!(f,"FILE CREATED"),
            AlertType::FileDeleted => write!(f,"FILE DELETED"),
            AlertType::HashChanged => write!(f,"HASH CHANGED"),
            AlertType::DirectoryCreated => write!(f,"DIRECTORY CREATED"),
            AlertType::DirectoryDeleted => write!(f,"DIRECTORY DELETED"),
            AlertType::PermissionChanged => write!(f,"PERMISSION CHANGED"),
            AlertType::OwnershipChanged => write!(f,"OWNERSHIP CHANGED")
        }
    }
}