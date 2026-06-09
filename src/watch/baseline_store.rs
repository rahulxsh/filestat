use std::path::{Path};
use std::fs;
use crate::watch::models::{BaseLineFile};
use anyhow::{Result as Res};
use serde_json::Result;

pub const BASELINE_FILE: &str = ".filestat-baseline.json";

pub fn create_baseline_file(content:&BaseLineFile,file_path:&str) {
    let path = Path::new(file_path);
    if let Ok(json_string) =  serde_json::to_string(&content) {
        let file_res = fs::write(path,json_string);
        match file_res {
            Ok(_d) => println!("File watch state saved"),
            Err(e) => println!("Error saving state:{}",e)
        }
    }
}

pub fn load_baseline_file(path:&str) -> Option<BaseLineFile> {
    if let Ok(file_content) = fs::read_to_string(path) {
        let baseline:Result<BaseLineFile> = serde_json::from_str(&file_content);
        match baseline {
            Ok(baseline) => {
                return  Some(baseline)
            }
            Err(_e) =>{
                println!("New baseline.json due to file load failure");
                return None
            }
        }
    }
    None
}

pub fn update_baseline_file(baseline:&BaseLineFile) -> Res<()> {
    if let Ok(json_string) = serde_json::to_string_pretty(&baseline) {
    fs::write(BASELINE_FILE,json_string)?;
    } else {
        println!("Baseline state update failed");
    }
    Ok(())
}