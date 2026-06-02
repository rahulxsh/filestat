use clap::{Parser,Subcommand};
use std::path::PathBuf;


#[derive(Parser,Debug)]
#[command(name="filestat")]
#[command(version="1.0")]
#[command(about="Filesystem statistics tool")]
pub struct Cli {
    #[command(subcommand)]
    pub command:Commands
}

#[derive(Subcommand,Debug)]
pub enum Commands {
    Scan {
        path:PathBuf,

        #[arg(long,default_value_t = 10)]
        top:usize,

        #[arg(long, default_value_t = false)]
        hidden:bool,

        #[arg(long,default_value_t = false)]
        print_extension:bool
    }
}