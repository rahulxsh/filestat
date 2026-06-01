mod clap_config;
mod models;
mod scanner;

use clap::Parser;
use std::path::{Path};
use crate::clap_config::{Cli, Commands};
use crate::scanner::scan;
use anyhow::Result;
fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, top, hidden } => {
            let path = Path::new(&path);
            let scan_result = scan(path,top,hidden)?;
        }
    }

    Ok(())
}

