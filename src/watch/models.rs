use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;
use crate::models::FilterConfig;
use crate::scanner::scan;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use crate::hashing::hash_file::hash_file;

pub enum EventTypes{
    CREATE,
    MODIFY,
    REMOVE
}

#[derive(Debug,Serialize,Deserialize)]
pub struct Baseline {
    pub hashes:HashMap<PathBuf,String>
}

impl BaseLineFile {
    pub fn build(path:&PathBuf) -> Result<Self> {
        let filter_config = FilterConfig {
            ignore:None,
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

    pub fn remove(&mut self, path:&Path) -> Result<()> {
        self.hashes.remove(path);
        Ok(())
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