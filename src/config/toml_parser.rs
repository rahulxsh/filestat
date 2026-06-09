use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

pub fn parse_config_file(path:String) -> Result<ConfigFile> {
    let value = read_to_string(path).context("Config file not found")?;
    let config:ConfigFile = toml::from_str(&value)?;

    let mut config_paths = ConfigFile {
        critical_paths:vec![],
        snapshot_paths:vec![]
    };

    for critical in config.critical_paths {
        let path = Path::new(&critical);
        if path.exists() {
            config_paths.critical_paths.push(path.to_path_buf())
        }else {
            println!("⚠️ --config file path doesn't exist make sure to check the absolute path");
            println!("PATH:{}",path.display());
        }
    }

    for snapshot in config.snapshot_paths {
        let path = Path::new(&snapshot);
        if path.exists() {
            config_paths.snapshot_paths.push(path.to_path_buf())
        }else {
            println!("⚠️ --config file path doesn't exist make sure to check the absolute path");
            println!("PATH:{}",path.display());
        }
    }

    Ok(config_paths)
}

#[derive(Serialize,Deserialize,Debug,Default)]
pub struct ConfigFile {
    pub critical_paths:Vec<PathBuf>,
    pub snapshot_paths:Vec<PathBuf>
}