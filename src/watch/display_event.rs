use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use notify::event::{CreateKind, ModifyKind};
use notify::{Event, EventKind};
use crate::hashing::hash_file::hash_file;
use crate::watch::baseline_store::update_baseline_file;
use crate::watch::models::{Alert, AlertType, BaseLineFile, BaselineFileInfo, EventTypes, Severity};
use anyhow::Result;
use crate::watch::alert::print_alert;

pub fn display_event(event:&Event, base_path:&Path, baseline:&mut BaseLineFile) {
    match event.kind {
        EventKind::Create(CreateKind::File) => {
            display(&event.paths,EventTypes::CREATE,base_path);
            for path in &event.paths {
                if should_ignore(path) {
                    continue;
                }
                match hash_file(path) {
                    Ok(hash) => {
                        let (size,modified)  = if let Ok(file_info) = get_fileinfo(&path) {
                            file_info
                        }else {
                            continue;
                        };
                        let size =
                        baseline.insert(path.clone(),BaselineFileInfo{
                            size,
                            hash:hash.to_string(),
                            modified
                        });
                    }
                    Err(e) => {
                        eprintln!(
                            "Unable to hash {}: {}",
                            path.display(),
                            e
                        );
                    }
                }
            }
            if let Err(_e) = update_baseline_file(&baseline) {
                println!("Baseline state update failed");
            }
        },
        EventKind::Create(CreateKind::Folder) => {
            display(&event.paths,EventTypes::CREATE,base_path);
        },
        EventKind::Modify(ModifyKind::Data(_)) => {
            display(&event.paths,EventTypes::MODIFY,base_path);
            for path in &event.paths {
                if should_ignore(path) {
                    continue;
                }
                if let Some(old_hash) = baseline.get(path) {
                    match hash_file(path) {
                        Ok(new_hash) => {
                            let new_hash = new_hash.to_string();
                            let (size,modified)  = if let Ok(file_info) = get_fileinfo(&path) {
                                file_info
                            } else {
                                continue;
                            };
                            if &old_hash.hash != &new_hash {
                                if let Ok(relative_path) = path.strip_prefix(base_path) {
                                    display_integrity(
                                        relative_path.to_path_buf(),
                                        old_hash.hash.clone(),
                                        new_hash.clone(),
                                        old_hash.size,
                                        size,
                                        old_hash.modified,
                                        modified,
                                        AlertType::HashChanged,
                                        Severity::High
                                    );
                                }
                            }
                            baseline.insert(
                                path.clone(),
                                BaselineFileInfo {
                                    size,
                                    hash:new_hash,
                                    modified
                                }
                            );
                        }
                        Err(err) => {
                            eprintln!(
                                "Unable to hash {}: {}",
                                path.display(),
                                err
                            );
                        }
                    }
                }
            }

            if let Err(_e) = update_baseline_file(&baseline) {
                println!("Baseline state update failed");
            }
        },

        EventKind::Remove(_) => {
            for path in &event.paths {
                let _res = baseline.remove(path);
            }
            if let Err(_e) = update_baseline_file(&baseline) {
                println!("Baseline state update failed");
            }
            display(&event.paths,EventTypes::REMOVE,base_path);
        },
        _ => {}
    }
}

fn display(paths:&[PathBuf],event_types: EventTypes,basepath:&Path) {
    for path in paths {
        if let Some(name) = path.file_name() {
            if name.to_string_lossy().ends_with('~') {
                continue;
            }
        }
        if let Ok(relative_path) = path.strip_prefix(basepath) {
            println!("{}: {}", event_types, relative_path.display());
        }
    }
}

impl Display for EventTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventTypes::CREATE => write!(f,"CREATE"),
            EventTypes::MODIFY => write!(f,"MODIFY"),
            EventTypes::REMOVE => write!(f,"REMOVE")
        }
    }
}

fn display_integrity(
    path:PathBuf,
    old_hash:String,
    new_hash:String,
    old_size:u64,
    new_size:u64,
    old_modified:u64,
    new_modified:u64,
    alert_type: AlertType,
    severity: Severity
) {
    let alert = Alert {
        timestamp:SystemTime::now(),
        alert_type,
        severity,
        path,
        old_size:Some(old_size),
        new_size:Some(new_size),
        old_hash:Some(old_hash),
        new_hash:Some(new_hash),
    };

    print_alert(&alert);
    let _ = old_modified;
    let _ = new_modified;
}


fn should_ignore(path: &Path) -> bool {
    path.file_name()
        .map(|name| name.to_string_lossy().ends_with('~'))
        .unwrap_or(false)
}


fn get_fileinfo(path:&Path) -> Result<(u64,u64)> {
    let metadata = std::fs::metadata(path)?;

    let size = metadata.len();
    let modified_time = metadata
        .modified()?
        .duration_since(UNIX_EPOCH)?
        .as_secs();

    Ok((size,modified_time))
}