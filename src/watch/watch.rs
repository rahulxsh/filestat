use notify::{Event,Result,RecursiveMode,Watcher};
use std::{path::Path,sync::mpsc};

pub fn watch_start() -> Result<()> {
    println!("Watch called");
    Ok(())
}