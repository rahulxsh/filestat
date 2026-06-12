use std::collections::HashMap;
use std::path::{PathBuf};
use crate::fim::watch::baseline_builder::build;
use crate::fim::watch::baseline_store::{create_baseline_file, load_baseline_file};
use crate::fim::watch::models::BaseLineFile;
use anyhow::{Context, Result};
use crate::config::toml_parser::ConfigFile;
use crate::fim::snapshot::model::SnapshotDiff;

pub fn get_snapshot_paths(snap_paths:Vec<PathBuf>) -> Vec<PathBuf> {
    let paths = snap_paths;

    let mut valid_paths = Vec::new();

    for path in paths {
        if let Ok(canonical) = std::fs::canonicalize(&path) {
            valid_paths.push(canonical);
        }
    }

    valid_paths
}
pub fn save_snapshot(filepath:&String,paths:ConfigFile){
    let paths_snap = get_snapshot_paths(paths.snapshot_paths);
    let mut ignore_paths = vec![];

    for i in paths.ignore{
        ignore_paths.push(i.to_string_lossy().to_string())
    }

    let mut map = BaseLineFile {
        hashes:HashMap::new()
    };

    for path in paths_snap {
        let baseline = build(&path,ignore_paths.clone());

        if let Ok(baseline) = baseline {
            map.hashes.extend(baseline.hashes)
        }
    }

   create_baseline_file(&map,filepath)
}

pub fn get_current_snapshot(snap_paths:ConfigFile) -> Result<BaseLineFile>{
    let paths_snap = get_snapshot_paths(snap_paths.snapshot_paths);

    let mut ignore_paths = vec![];

    for i in snap_paths.ignore{
        ignore_paths.push(i.to_string_lossy().to_string())
    }

    let mut map = BaseLineFile {
        hashes:HashMap::new()
    };

    for path in paths_snap {
        let baseline = build(&path,ignore_paths.clone());

        if let Ok(baseline) = baseline {
            map.hashes.extend(baseline.hashes)
        }
    }

    Ok(map)
}


pub fn snapshot_diff(path:&str,snap_paths:ConfigFile) -> Result<SnapshotDiff> {

    let snapshot = load_baseline_file(path)
        .context("Unable to load old snapshot for diff")?;
    let current_snapshot = get_current_snapshot(snap_paths)
        .context("Unable to create latest snapshot")?;

    let mut res = SnapshotDiff {
        added:vec![],
        deleted:vec![],
        modified:vec![]
    };

    for old_paths in snapshot.hashes.keys() {
        if !current_snapshot.hashes.contains_key(old_paths) {
                res.deleted.push(old_paths.clone())
        }
    }

    for current_path in current_snapshot.hashes.keys() {
        if !snapshot.hashes.contains_key(current_path) {
            res.added.push(current_path.clone())
        }
    }

    for (path,info) in snapshot.hashes.iter() {
        if let Some(new_file) = current_snapshot.hashes.get(path) {
            if new_file.hash != info.hash {
                res.modified.push(path.clone())
            }
        }
    }


    println!("Snapshot Diff:");
    println!("Added:{}",res.added.len());
    println!("Deleted:{}",res.deleted.len());
    println!("Modified:{}",res.modified.len());

    Ok(res)
}


pub fn print_snap_shot_diff_files(snapshot_diff:&SnapshotDiff) {
    let added_len = snapshot_diff.added.len();
    let deleted_len = snapshot_diff.deleted.len();
    let modified_len = snapshot_diff.modified.len();
    println!("\n");
   if !snapshot_diff.added.is_empty() {
       println!("Added:({})",added_len);
       for path in snapshot_diff.added.iter() {
           println!("{}",path.display());
       }
       println!("\n");
   }

    if !snapshot_diff.deleted.is_empty() {
        println!("Deleted:({})",deleted_len);
        for path in snapshot_diff.deleted.iter() {
            println!("{}",path.display());
        }
        println!("\n");
    }

    if !snapshot_diff.modified.is_empty() {
        println!("Modified({})",modified_len);
        for path in snapshot_diff.modified.iter() {
            println!("{}",path.display());
        }
    }
}