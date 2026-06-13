use crate::agent::agent::{AgentRuntime, Module};
use crate::fim::watch::watch::watch_start;

pub struct FimModule;

impl Module for FimModule {
     fn start(runtime:&AgentRuntime) -> anyhow::Result<()> {
        if !runtime.config.monitor_paths.is_empty() {
            watch_start(&runtime.config,&runtime.conn)?;
        } else {
            println!("Given Path doesn't exist");
        }
        Ok(())
    }
}