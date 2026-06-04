mod clap_config;
mod models;
mod scanner;
mod stats;
mod files;
mod metadata;
mod filters;
mod utils;
mod hashing;

use std::collections::HashMap;
use clap::Parser;
use std::path::{Path, PathBuf};
use crate::clap_config::{Cli, Commands};
use crate::scanner::scan;
use anyhow::{bail, Result};
use crate::files::csv::export_csv;
use crate::files::json::{json_stats, save_json};
use crate::hashing::get_duplicates::{get_full_duplicates};
use crate::models::FilterConfig;
use crate::stats::generate_stats;
use std::time::Instant;

fn main() -> Result<()> {
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

            let stats_report = generate_stats(&mut files,top);

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

            if duplicate {
                let duplicate_start_time = Instant::now();
                let mut unique_file_size_map: HashMap<u64, Vec<PathBuf>> = HashMap::new();
                for i in &mut files.files {
                    let path = std::mem::take(&mut i.path);
                    unique_file_size_map.entry(i.size).or_default().push(path);
                }

                println!("-----------------------");
                println!("Unique file size mapping done ✅");
                println!("-----------------------");

                let duplicates = get_full_duplicates(unique_file_size_map)?;
                let mut count = 0;

                for (_key, val) in duplicates {
                    if val.len() > 1 {
                        count += 1;
                    }
                }
                println!("Total duplicate groups count: {}", count);
                let duplicate_total_time = duplicate_start_time.elapsed();
                println!("Time taken in duplication check: {:?}", duplicate_total_time);
            }
        let duration = start_time.elapsed();
            println!("Total time:{:?}",duration);
        }
    }

    Ok(())
}

