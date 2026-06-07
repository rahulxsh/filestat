use notify::{Event,Result,RecursiveMode,Watcher};
use std::sync::mpsc;
use std::path::{PathBuf};
use crate::watch::baseline_builder::build;
use crate::watch::baseline_store::{create_baseline_file, load_baseline_file};
use crate::watch::display_event::display_event;

pub fn watch_start(path:&PathBuf) -> Result<()> {
    let mut baseline = if let Some(base_line) = load_baseline_file() {
        println!("Loaded existing baseline");
        base_line
    }else {
        println!("Building baseline for integrity...");
        let baseline = build(path).expect("Baseline build failed");

        create_baseline_file(&baseline);

        println!("Baseline created");
        baseline
    };
    println!("Success");

    let (tx,rx) = mpsc::channel::<Result<Event>>();
    let basepath = path.canonicalize()?;

    let mut watcher = notify::recommended_watcher(tx)?;


    watcher.watch(path, RecursiveMode::Recursive)?;
    println!("Watching:{:?}",&path);

    for res  in rx {
        match res {
            Ok(event) =>{
                display_event(&event,&basepath,&mut baseline);
            },
            Err(e) => println!("Watch error:{}",e),
        }
    }
    Ok(())
}