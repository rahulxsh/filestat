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
        print_extension:bool,

        #[arg(long, default_value_t = false)]
        largest_files:bool,

        #[arg(long, default_value_t = false)]
        size:bool,

        #[arg(long, default_value_t = false)]
        total:bool,

        #[arg(long, num_args = 0..=1 )]
        json: Option<Option<PathBuf>>,

        #[arg(long, default_value_t = false )]
        csv:bool,

        #[arg(long)]
        ext:Option<Vec<String>>,

        #[arg(long, num_args = 0..=1 )]
        min_size:Option<String>,

        #[arg(long, num_args = 0..=1 )]
        max_size:Option<String>,

        #[arg(long)]
        ignore:Option<Vec<String>>,
        
        #[arg(long, default_value_t = false )]
        duplicate:bool
    },
    Watch {
        #[arg(long)]
        config:String,
    },
    Snapshot {
        #[command(subcommand)]
        command:SnapshotCommands,

        #[arg(long)]
        config:String,

        #[arg(long, default_value_t = false )]
        show_paths:bool,
    },
    Alerts {
        #[arg(long,default_value_t = 20)]
        limit:usize
    }
}

#[derive(Debug,Clone,Subcommand)]
pub enum SnapshotCommands {
    Save ,
    Diff
}