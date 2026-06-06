use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use notify::event::ModifyKind;
use notify::{Event, EventKind};
use crate::watch::models::EventTypes;

pub fn display_event(event:&Event,base_path:&Path) {
    match event.kind {
        EventKind::Create(_) => {
            display(&event.paths,EventTypes::CREATE,base_path);
        },
        EventKind::Modify(ModifyKind::Data(_)) => {
            display(&event.paths,EventTypes::MODIFY,base_path);
        },
        EventKind::Remove(_) => {
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