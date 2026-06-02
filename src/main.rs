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
        Commands::Scan { path, top, hidden,print_extension,largest_files } => {
            let path = Path::new(&path);
            let mut files = scan(path,hidden)?;
            println!("Total Files:{}\nTotal Dirs:{}",files.files.len(),files.total_dirs);

            let extensions = stats::extension_count(&files.files,"rs");

            if print_extension {
                for (ext,count) in extensions {
                    println!("{:?}:{}",ext,count);
                }
            }

           if largest_files {
               let largest_files = stats::largets_files(&mut files.files,top);
               println!("Largest Files:");
               for file in largest_files {
                   println!("PATH:{:?}, SIZE:{}",file.path,file.size);
               }
           }

        }
    }

    Ok(())
}

