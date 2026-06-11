use std::collections::HashMap;
use notify::{Event, Result, RecursiveMode, Watcher};
use std::sync::mpsc;
use std::path::{Path};
use rusqlite::Connection;
use crate::config::toml_parser::ConfigFile;
use crate::watch::baseline_builder::build;
use crate::watch::baseline_store::{create_baseline_file, load_baseline_file, BASELINE_FILE};
use crate::watch::critical_path::{get_critical_paths};
use crate::watch::display_event::display_event;
use crate::watch::models::BaseLineFile;

pub fn watch_start(config_paths:&ConfigFile,conn:&Connection) -> Result<()> {
    let mut baseline = if let Some(base_line) = load_baseline_file(BASELINE_FILE) {
        println!("Loaded existing baseline.json");
        base_line
    }else {
        println!("Building baseline for integrity...");
        let mut map = BaseLineFile {
            hashes:HashMap::new()
        };

        let baseline_config_paths = config_paths.monitor_paths.clone();
        let mut ignore_paths = vec![];

        if !config_paths.ignore.is_empty() {
            for p in config_paths.ignore.iter() {
                ignore_paths.push(p.to_string_lossy().to_string());
            }
        }
        for p in baseline_config_paths {
            let baseline = build(&p,ignore_paths.clone()).expect("Baseline build failed");

            map.hashes.extend(baseline.hashes)
        }

        create_baseline_file(&map,BASELINE_FILE);

        println!("Baseline created");
        map
    };
    println!("Success");

    let (tx,rx) = mpsc::channel::<Result<Event>>();
    let mut basepath = vec![];
    let mut put_c_path = vec![];
    let channel_config_paths = config_paths.monitor_paths.clone();
    for p in channel_config_paths {
        let canon_path= p.canonicalize()?;
        put_c_path.push(canon_path);
    }

    for c_p in put_c_path.iter() {
        basepath.push(c_p.as_path());
    }
    let critical_paths = get_critical_paths(config_paths.critical_paths.clone());

    let mut watcher = notify::recommended_watcher(tx)?;


   for p in config_paths.monitor_paths.iter() {
       let path = p.clone();
       if path == Path::new("/") {
           return Err(notify::Error::new(notify::ErrorKind::Generic(String::from("Root path can not be monitored as it contain os secret paths"))))
       }
       if path.exists() {
           println!("Watching:{:?}",&p);
           watcher.watch(path.as_path(), RecursiveMode::Recursive)?;
       }else {
           println!("⚠️Skipping:{},NOT EXIST",path.display());
       }
   }

    for res  in rx {
        match res {
            Ok(event) =>{
                for path in &event.paths {
                }
                display_event(&event,&basepath,&mut baseline,&critical_paths,&config_paths,&conn);
            },
            Err(e) => println!("Watch error:{}",e),
        }
    }
    Ok(())
}