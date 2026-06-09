use std::path::{Path, PathBuf};
use anyhow::Result;
use crate::watch::models::Severity;

pub struct CriticalPaths {
    pub paths: Vec<PathBuf>,
}

pub fn get_critical_paths(paths:Vec<PathBuf>) -> CriticalPaths {
    let critical_paths = CriticalPaths {
        //TODO: Replace this with actual config file
        paths
    };

    let mut canonical_paths = CriticalPaths {
        paths:Vec::new()
    };

    for path in critical_paths.paths {
        let path_r = std::fs::canonicalize(&path);
        match path_r {
            Ok(value) => canonical_paths.paths.push(value),
            Err(e) => {
                eprintln!(
                    "Warning: Critical path '{}' skipped: {}",
                    path.display(),
                    e
                );
            }
        }
    }

    canonical_paths
}

pub fn is_critical_path(path: &Path,critical_paths: &CriticalPaths) -> Result<bool> {
    for critical in critical_paths.paths.iter() {
        if path.starts_with(&critical) {
            return Ok(true);
        }
    }

    Ok(false)
}

pub fn get_severity_level(
    path: &Path,
    default_severity: Severity,
    critical_paths: &CriticalPaths,
) -> Severity {
    match is_critical_path(path,critical_paths) {
        Ok(true) => Severity::Critical,
        Ok(false) => default_severity,
        Err(_) => default_severity,
    }
}