use std::collections::HashMap;
use std::path::{Path, PathBuf};
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

impl Baseline {
    pub fn build(path:&PathBuf) -> Result<Self> {
        let filter_config = FilterConfig {
            ignore:None,
            ext:None,
            min_size:None,
            max_size:None
        };
        let scanned_files = scan(path,true,filter_config)?;
        let mut map:HashMap<PathBuf,String> = HashMap::new();

        for file in scanned_files.files {
            let hash = hash_file(&file.path)?;
            let canonical = file.path.canonicalize()?;
            map.insert(canonical,hash.to_string());
        }

        Ok(Self {
            hashes:map
        })
    }

    pub fn get(&self,path:&Path) -> Option<&String> {
        let hash = self.hashes.get(path);
        hash
    }

    pub fn insert(&mut self, path:PathBuf, hash:String) {
        self.hashes.insert(path,hash);
    }

    pub fn remove(&mut self, path:&Path) -> Result<()> {
        self.hashes.remove(path);
        Ok(())
    }
}