use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use notify::event::{CreateKind, ModifyKind};
use notify::{Event, EventKind};
use crate::hashing::hash_file::hash_file;
use crate::watch::models::{Baseline, EventTypes};

pub fn display_event(event:&Event,base_path:&Path,baseline:&mut Baseline) {
    match event.kind {
        EventKind::Create(CreateKind::File) => {
            display(&event.paths,EventTypes::CREATE,base_path);
            for path in &event.paths {
                if should_ignore(path) {
                    continue;
                }
                match hash_file(path) {
                    Ok(hash) => {
                        baseline.insert(path.clone(),hash.to_string())
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
                            if old_hash != &new_hash.to_string() {
                                if let Ok(relative_path) = path.strip_prefix(base_path) {
                                    display_integrity(relative_path,old_hash, &new_hash);
                                }
                            }
                            baseline.insert(
                                path.clone(),
                                new_hash.to_string()
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
        },
        EventKind::Remove(_) => {
            for path in &event.paths {
                let _res = baseline.remove(path);
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

fn display_integrity(path:&Path,old_hash:&str,new_hash:&str) {
    println!("Integrity Changed: {}",path.display());
    println!("OLD Hash: {}",old_hash);
    println!("NEW Hash: {}",new_hash);
}


fn should_ignore(path: &Path) -> bool {
    path.file_name()
        .map(|name| name.to_string_lossy().ends_with('~'))
        .unwrap_or(false)
}// test
