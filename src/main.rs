mod clap_config;
mod config;
mod storage;
mod fim;
mod agent;
mod process_monitor;

use std::collections::HashMap;
use std::fs;
use clap::Parser;
use std::path::{Path, PathBuf};
use crate::clap_config::{Cli, Commands, SnapshotCommands};
use fim::scanner::scan;
use anyhow::{bail, Result};
use fim::files::csv::export_csv;
use fim::files::json::{json_stats, save_json};
use fim::hashing::get_duplicates::get_full_duplicates;
use fim::models::{FilterConfig, PerformanceMetrics};
use fim::stats::generate_stats;
use std::time::Instant;
use rusqlite::Connection;
use crate::fim::alerts::alerts::{alerts, display_alerts};
use crate::config::toml_parser::parse_config_file;
use fim::snapshot::snapshot::{print_snap_shot_diff_files, save_snapshot, snapshot_diff};
use crate::storage::db::{get_db_path, init_db};
use fim::watch::watch::watch_start;
use crate::agent::agent::Agent;

#[cfg(target_os = "macos")]
use crate::process_monitor::providers::esf_provider::esf;

#[cfg(target_os = "linux")]
use crate::process_monitor::providers::auditd::netlink_audit;

fn main() -> Result<()> {
    #[cfg(target_os = "linux")]
    netlink_audit()?;

    #[cfg(target_os = "macos")]
    esf()?;

    let db_path = get_db_path();
    let conn = Connection::open(db_path)?;
    init_db(&conn)?;


    let cli = Cli::parse();

    match cli.command {
        Commands::Scan {
            path,
            top,
            hidden,
            print_extension,
            largest_files,
            size,
            total,
            json,
            csv,
            ext,
            max_size,
            min_size,
            ignore,
            duplicate
        } => {
            let start_time = Instant::now();
            let path = Path::new(&path);
            let filters = FilterConfig {
                ext,
                min_size,
                max_size,
                ignore
            };
            let mut files = scan(path,hidden,filters)?;
            let duration = start_time.elapsed().as_secs_f64();

            let files_per_second = if duration > 0.0 {
                files.files.len() as f64 / duration
            } else {
                0.0
            };

            let performance_metrics = PerformanceMetrics {
                duration_secs:duration,
                files_scanned:files.files.len(),
                files_per_second
            };

            let stats_report = generate_stats(&mut files,top,&performance_metrics);

            if total {
                println!("Total Files:{}\nTotal Dirs:{}\n",stats_report.total_files,stats_report.total_dirs);
            }

            if print_extension {
                for (ext,count) in &stats_report.extension_count {
                    println!("{:?}:{}",ext,count);
                }
            }

           if largest_files {
               println!("Largest Files:");
               for file in &stats_report.largest_files {
                   println!("PATH:{:?}, SIZE:{} bytes",file.path,file.size);
               }
           }

            if size {
                println!(
                    "Total files size:{:.2} bytes\nAverage File Size:{:.2} bytes",
                    &stats_report.total_size,&stats_report.average_size
                );
            }

            match json {
                None => {},
                Some(None) => {
                    println!("{:?}",json_stats(&stats_report));
                },
                Some(Some(input_file_path)) => {
                    if input_file_path.extension() == Some("json".as_ref()){
                        save_json(&input_file_path,&stats_report)?;
                    }else{
                        bail!("Please provide a correct json file name");
                    }
                }
            }

            if csv {
                export_csv(&stats_report)?;
            }
            println!("\n ✅ Files Scan Summary\n--------------\nFiles Scanned: {}\nDuration: {}s\nFiles/sec: {}",
                     performance_metrics.files_scanned, performance_metrics.duration_secs, performance_metrics.files_per_second);


            if duplicate {
                let duplicate_start_time = Instant::now();
                let mut unique_file_size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
                for i in &mut files.files {
                    let path = std::mem::take(&mut i.path);
                    unique_file_size_map.entry(i.size).or_default().push(path);
                }

                let (duplicates,total_expected_duplicate_files) = get_full_duplicates(unique_file_size_map)?;
                let duplicate_total_time = duplicate_start_time.elapsed().as_secs_f64();
                let mut count = 0;

                for (_key, val) in duplicates {
                    if val.len() > 1 {
                        count += 1;
                    }
                }
                let per_second_file_scan_duplicate = total_expected_duplicate_files as f64 / duplicate_total_time;

                println!("\n ✅ Duplicate Scan Summary\n--------------");
                println!("Expected duplicate files: {}", total_expected_duplicate_files);
                println!("Total duplicate files: {}", count);
                println!("Duration: {}s", duplicate_total_time);
                println!("Hashes/sec: {}\n",per_second_file_scan_duplicate);
            }
        }

        Commands::Watch {
            config
        } => {
            let config_data = parse_config_file(config)?;
            if !config_data.monitor_paths.is_empty() {
                watch_start(&config_data,&conn)?;
            } else {
                println!("Given Path doesn't exist");
            }
        }

        Commands::Snapshot {command,show_paths,config} => {
            fs::create_dir_all(".snapshots")?;
            let config_data = parse_config_file(config)?;
            match command {
                SnapshotCommands::Save  => {
                    let path = String::from("./.snapshots/snapshot.json");
                    save_snapshot(&path,config_data);
                }

                SnapshotCommands::Diff  => {
                    let path = "./.snapshots/snapshot.json";
                    let diff = snapshot_diff(path,config_data)?;
                    if show_paths {
                        print_snap_shot_diff_files(&diff)
                    }
                }
            }
        }

        Commands::Alerts {limit} => {
            let limit_ = limit as i64;
            let alerts = alerts(&conn,limit_)?;
            for alert in alerts {
               display_alerts(alert);
            }
        }

        Commands::Agent => {
            Agent::start()?;
        }
    }

    Ok(())
}