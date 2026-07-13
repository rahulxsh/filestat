use std::time::SystemTime;

#[derive(Debug)]
pub struct ProcessEvent {
    pub timestamp: SystemTime,
    pub pid: u32,
    pub ppid: u32,
    pub uid:u32,
    pub user: Option<String>,
    pub executable_path: String,
    pub command_line: Vec<String>,
}

pub trait ProcessProvider {
    fn start(&self) -> anyhow::Result<()>;
}