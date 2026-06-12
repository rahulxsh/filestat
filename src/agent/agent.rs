pub struct Agent;
use anyhow::Result;
use rusqlite::Connection;
use crate::config::toml_parser::parse_config_file;
use crate::fim::watch::watch::watch_start;
use crate::storage::db::init_db;

impl Agent {
    pub fn start(conn:&Connection) -> Result<()>{
        println!("Agent Starting...");
        init_db(&conn)?;

        const DEFAULT_CONFIG: &str = "./config/agent.toml";

        let config_data = parse_config_file(DEFAULT_CONFIG.to_string())?;

        if !config_data.monitor_paths.is_empty() {
            watch_start(&config_data,&conn)?;
        } else {
            println!("Given Path doesn't exist");
        }
        Ok(())
    }
}