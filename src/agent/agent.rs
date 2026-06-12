pub struct Agent;
use anyhow::Result;
use rusqlite::Connection;
use crate::config::toml_parser::{parse_config_file, ConfigFile};
use crate::fim::module::FimModule;
use crate::storage::db::{get_db_path, init_db};


pub struct AgentRuntime {
    pub config: ConfigFile,
    pub conn: Connection,
}

impl Agent {
    pub fn start() -> Result<()> {
        const DEFAULT_CONFIG: &str = "./config/agent.toml";

        let config = parse_config_file(DEFAULT_CONFIG.to_string())?;

        let db_path = get_db_path();
        let conn = Connection::open(db_path)?;
        init_db(&conn)?;

        let runtime = AgentRuntime {
            config,
            conn,
        };

        FimModule::start(&runtime)?;

        Ok(())
    }
}