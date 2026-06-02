mod clap_config;
mod models;
mod scanner;
mod stats;

use clap::Parser;
use std::path::{Path};
use crate::clap_config::{Cli, Commands};
use crate::scanner::scan;
use anyhow::Result;
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, top, hidden,print_extension } => {
            let path = Path::new(&path);
            let files = scan(path,hidden)?;
            let extensions = stats::extension_count(&files,"rs");
            let total_files_and_dir_count = stats::total_file_and_dirs(&files);

            println!("Total Files: {} \nTotal Dirs:{}",
                     total_files_and_dir_count.total_files,total_files_and_dir_count.total_dirs
            );

            if print_extension {
                for (ext,count) in extensions {
                    println!("{:?}:{}",ext,count);
                }
            }
        }
    }

    Ok(())
}

