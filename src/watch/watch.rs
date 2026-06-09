use notify::{Event,Result,RecursiveMode,Watcher};
use std::sync::mpsc;
use std::path::{PathBuf};
use anyhow::bail;
use crate::watch::baseline_builder::build;
use crate::watch::baseline_store::{create_baseline_file, load_baseline_file, BASELINE_FILE};
use crate::watch::critical_path::get_critical_paths;
use crate::watch::display_event::display_event;

pub fn watch_start(path:&PathBuf) -> Result<()> {
    let mut baseline = if let Some(base_line) = load_baseline_file() {
        println!("Loaded existing baseline.json");
        base_line
    }else {
        println!("Building baseline.json for integrity...");
        let baseline = build(path).expect("Baseline build failed");

        create_baseline_file(&baseline,BASELINE_FILE);

        println!("Baseline created");
        baseline
    };
    println!("Success");

    let (tx,rx) = mpsc::channel::<Result<Event>>();
    let basepath = path.canonicalize()?;
    let critical_paths = get_critical_paths();

    let mut watcher = notify::recommended_watcher(tx)?;


    watcher.watch(path, RecursiveMode::Recursive)?;
    println!("Watching:{:?}",&path);

    for res  in rx {
        match res {
            Ok(event) =>{
                display_event(&event,&basepath,&mut baseline,&critical_paths);
            },
            Err(e) => println!("Watch error:{}",e),
        }
    }
    Ok(())
}