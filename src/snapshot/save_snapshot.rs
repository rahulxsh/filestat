use std::collections::HashMap;
use std::path::{PathBuf};
use crate::watch::baseline_builder::build;
use crate::watch::baseline_store::create_baseline_file;
use crate::watch::models::BaseLineFile;

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