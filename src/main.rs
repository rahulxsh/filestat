mod clap_config;
mod models;
mod scanner;

use clap::Parser;
use std::path::{Path};
use crate::clap_config::{Cli, Commands};
use crate::scanner::scan;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, top, hidden } => {
            println!("Path: {:?}", path);
            println!("Top files: {}", top);
            println!("Include hidden: {}", hidden);

            let path = Path::new(&path);
            scan(path,top,hidden);
        }
    }
}

