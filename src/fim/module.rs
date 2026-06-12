use crate::agent::agent::AgentRuntime;
use crate::fim::watch::watch::watch_start;

pub struct FimModule;

impl FimModule {
    pub fn start(runtime:&AgentRuntime) -> anyhow::Result<()> {
        if !runtime.config.monitor_paths.is_empty() {
            watch_start(&runtime.config,&runtime.conn)?;
        } else {
            println!("Given Path doesn't exist");
        }
        Ok(())
    }
}