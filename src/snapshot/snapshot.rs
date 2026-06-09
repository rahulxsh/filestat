use std::collections::HashMap;
use std::path::{PathBuf};
use crate::watch::baseline_builder::build;
use crate::watch::baseline_store::{create_baseline_file, load_baseline_file};
use crate::watch::models::BaseLineFile;
use anyhow::{Context, Result};

pub fn get_snapshot_paths() -> Vec<PathBuf> {
    let paths = vec![
        PathBuf::from("./src/watch"),
        PathBuf::from("./src/hashing"),
    ];

    let mut valid_paths = Vec::new();

    for path in paths {
        if let Ok(canonical) = std::fs::canonicalize(&path) {
            valid_paths.push(canonical);
        }
    }

    valid_paths
}
pub fn save_snapshot(filepath:&String){
    let paths = get_snapshot_paths();

    let mut map = BaseLineFile {
        hashes:HashMap::new()
    };

    for path in paths {
        let baseline = build(&path);

        if let Ok(baseline) = baseline {
            map.hashes.extend(baseline.hashes)
        }
    }

   create_baseline_file(&map,filepath)
}

pub fn get_current_snapshot() -> Result<BaseLineFile>{
    let paths = get_snapshot_paths();

    let mut map = BaseLineFile {
        hashes:HashMap::new()
    };

    for path in paths {
        let baseline = build(&path);

        if let Ok(baseline) = baseline {
            map.hashes.extend(baseline.hashes)
        }
    }

    Ok(map)
}


pub fn snapshot_diff(path:&str) -> Result<()> {

    let snapshot = load_baseline_file(path)
        .context("Unable to load old snapshot for diff")?;
    let current_snapshot = get_current_snapshot()
        .context("Unable to create latest snapshot")?;

    let mut deleted = vec![];
    let mut added = vec![];
    let mut modified = vec![];

    for old_paths in snapshot.hashes.keys() {
        if !current_snapshot.hashes.contains_key(old_paths) {
                deleted.push(old_paths)
        }
    }

    for current_path in current_snapshot.hashes.keys() {
        if !snapshot.hashes.contains_key(current_path) {
            added.push(current_path)
        }
    }

    for (path,info) in snapshot.hashes.iter() {
        if let Some(new_file) = current_snapshot.hashes.get(path) {
            if new_file.hash != info.hash {
                modified.push(path)
            }
        }
    }

    println!("Added:{}",added.len());
    println!("Deleted:{}",deleted.len());
    println!("Modified:{}",modified.len());

    Ok(())
}