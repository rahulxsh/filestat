use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use notify::event::{CreateKind, ModifyKind, RemoveKind};
use notify::{Event, EventKind};
use crate::fim::hashing::hash_file::hash_file;
use crate::fim::watch::baseline_store::update_baseline_file;
use crate::fim::watch::models::{Alert, AlertType, BaseLineFile, BaselineFileInfo, EventTypes, Severity};
use anyhow::Result;
use rusqlite::Connection;
use crate::config::toml_parser::ConfigFile;
use crate::storage::db::insert_alert;
use crate::fim::watch::alert::print_alert;
use crate::fim::watch::critical_path::{get_severity_level, CriticalPaths};

pub fn display_event(
    event:&Event, 
    base_path:&Vec<&Path>, 
    baseline:&mut BaseLineFile,
    critical_paths: &CriticalPaths,
    ignore_paths:&ConfigFile,
    conn:&Connection
) {
    if event.paths.iter().any(|p| {
        p.to_string_lossy().to_string().ends_with(".filestat-baseline.json")
    }){
        return;
    }

    match event.kind {
        EventKind::Create(CreateKind::File) => {
            // display(&event.paths,EventTypes::CREATE,base_path);
            for path in &event.paths {
                if should_ignore(path,ignore_paths) {
                    continue;
                }
                match hash_file(path) {
                    Ok(hash) => {
                        let (size, modified) = if let Ok(file_info) = get_fileinfo(&path) {
                            file_info
                        } else {
                            continue;
                        };

                        for &p in base_path.iter() {
                            if let Ok(relative_path) = path.strip_prefix(p) {
                                let severity: Severity = get_severity_level(path, Severity::Medium, critical_paths);
                                display_integrity(
                                    relative_path.to_path_buf(),
                                    None,
                                    Some(hash.to_string().clone()),
                                    None,
                                    Some(size),
                                    None,
                                    Some(modified),
                                    AlertType::FileCreated,
                                    severity.clone()
                                );
                                let alert = Alert {
                                    timestamp:SystemTime::now(),
                                    alert_type:AlertType::FileCreated,
                                    severity,
                                    path:relative_path.to_path_buf(),
                                    old_size:None,
                                    new_size:Some(size),
                                    old_hash:None,
                                    new_hash:Some(hash.to_string())
                                };
                                
                                let _ = insert_alert(conn,&alert);
                            }
                        }
                        baseline.insert(path.clone(), BaselineFileInfo {
                            size,
                            hash: hash.to_string(),
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
            // display(&event.paths,EventTypes::CREATE,base_path);

            for path in &event.paths {
                if should_ignore(path, ignore_paths) {
                    continue;
                }
                for &p in base_path.iter() {
                    if let Ok(relative_path) = path.strip_prefix(p) {
                        let severity = get_severity_level(path, Severity::Low, critical_paths);
                        display_integrity(
                            relative_path.to_path_buf(),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            AlertType::DirectoryCreated,
                            severity.clone()
                        );

                        let alert = Alert {
                            timestamp:SystemTime::now(),
                            alert_type:AlertType::DirectoryCreated,
                            severity,
                            path:relative_path.to_path_buf(),
                            old_size:None,
                            new_size:None,
                            old_hash:None,
                            new_hash:None
                        };

                        let _ = insert_alert(conn,&alert);
                    }
                }
            }
        },
        EventKind::Modify(ModifyKind::Data(_)) => {
            // display(&event.paths,EventTypes::MODIFY,base_path);
            for path in &event.paths {
                if should_ignore(path, ignore_paths) {
                    continue;
                }
                if should_ignore(path,ignore_paths) {
                    continue;
                }
                if let Some(old_hash) = baseline.get(path) {
                    match hash_file(path) {
                        Ok(new_hash) => {
                            let new_hash = new_hash.to_string();
                            let (size, modified) = if let Ok(file_info) = get_fileinfo(&path) {
                                file_info
                            } else {
                                continue;
                            };
                            if &old_hash.hash != &new_hash {
                                for &p in base_path.iter() {
                                    if let Ok(relative_path) = path.strip_prefix(p) {
                                        let severity = get_severity_level(path, Severity::High, critical_paths);
                                        display_integrity(
                                            relative_path.to_path_buf(),
                                            Some(old_hash.hash.clone()),
                                            Some(new_hash.clone()),
                                            Some(old_hash.size),
                                            Some(size),
                                            Some(old_hash.modified),
                                            Some(modified),
                                            AlertType::HashChanged,
                                            severity.clone()
                                        );

                                        let alert = Alert {
                                            timestamp:SystemTime::now(),
                                            alert_type:AlertType::HashChanged,
                                            severity,
                                            path:relative_path.to_path_buf(),
                                            old_size:Some(old_hash.size),
                                            new_size:Some(size),
                                            old_hash:Some(old_hash.hash.clone()),
                                            new_hash:Some(new_hash.to_string())
                                        };

                                        let _ = insert_alert(conn,&alert);
                                    }
                                }
                            }
                            baseline.insert(
                                path.clone(),
                                BaselineFileInfo {
                                    size,
                                    hash: new_hash,
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

        EventKind::Remove(RemoveKind::File) => {
            for path in &event.paths {
                if should_ignore(path, ignore_paths) {
                    continue;
                }
                if let Some(res) = baseline.remove(path) {
                    for &p in base_path.iter() {
                        if let Ok(relative_path) = path.strip_prefix(p) {
                            let severity = get_severity_level(path, Severity::High, critical_paths);
                            display_integrity(
                                relative_path.to_path_buf(),
                                Some(res.hash.clone()),
                                None,
                                Some(res.size),
                                None, Some(res.modified), None,
                                AlertType::FileDeleted,
                                severity.clone()
                            );

                            let alert = Alert {
                                timestamp:SystemTime::now(),
                                alert_type:AlertType::FileDeleted,
                                severity,
                                path:relative_path.to_path_buf(),
                                old_size:Some(res.size),
                                new_size:None,
                                old_hash:Some(res.hash.clone()),
                                new_hash:None
                            };

                            let _ = insert_alert(conn,&alert);
                        }
                    }
                }
            }
            if let Err(_e) = update_baseline_file(&baseline) {
                println!("Baseline state update failed");
            }
            // display(&event.paths,EventTypes::REMOVE,base_path);
        },
        EventKind::Remove(RemoveKind::Folder) => {
            for path in &event.paths {
                if should_ignore(path, ignore_paths) {
                    continue;
                }
                let _ = baseline.remove(path);

                for &p in base_path.iter() {
                    if let Ok(relative_path) = path.strip_prefix(p) {
                        let severity = get_severity_level(path, Severity::Medium, critical_paths);
                        display_integrity(
                            relative_path.to_path_buf(),
                            None,
                            None,
                            None,
                            None,
                            None,
                            None,
                            AlertType::DirectoryDeleted,
                            severity.clone(),
                        );

                        let alert = Alert {
                            timestamp:SystemTime::now(),
                            alert_type:AlertType::DirectoryDeleted,
                            severity,
                            path:relative_path.to_path_buf(),
                            old_size:None,
                            new_size:None,
                            old_hash:None,
                            new_hash:None
                        };

                        let _ = insert_alert(conn,&alert);
                    }
                }
            }

            let _ = update_baseline_file(baseline);
            // display(&event.paths,EventTypes::REMOVE,base_path);
        },
        _ => {}
    }
}

#[allow(unused)]
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
    old_hash:Option<String>,
    new_hash:Option<String>,
    old_size:Option<u64>,
    new_size:Option<u64>,
    old_modified:Option<u64>,
    new_modified:Option<u64>,
    alert_type: AlertType,
    severity: Severity
) {
    let alert = Alert {
        timestamp:SystemTime::now(),
        alert_type,
        severity,
        path,
        old_size,
        new_size,
        old_hash,
        new_hash,
    };

    print_alert(&alert);
    let _ = old_modified;
    let _ = new_modified;
}


fn should_ignore(path: &Path, config: &ConfigFile) -> bool {
    if path.file_name()
        .map(|name| name.to_string_lossy().ends_with('~'))
        .unwrap_or(false)
    {
        return true;
    }

    path.components().any(|component| {
        let name = component.as_os_str().to_string_lossy();

        config.ignore.iter().any(|pattern| {
            pattern.to_string_lossy() == name
        })
    })
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