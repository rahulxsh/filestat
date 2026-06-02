mod clap_config;
mod models;
mod scanner;
mod stats;
mod files;

use clap::Parser;
use std::path::{Path};
use crate::clap_config::{Cli, Commands};
use crate::scanner::scan;
use anyhow::Result;
use crate::files::json::json_export;
use crate::stats::generate_stats;
use std::fs;

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
            json
        } => {
            let path = Path::new(&path);
            let mut files = scan(path,hidden)?;

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

            if json {
                fs::write("stats.json",json_export(&stats_report)?)?;
            }

        }
    }

    Ok(())
}

