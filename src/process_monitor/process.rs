use std::time::SystemTime;

#[derive(Debug)]
pub struct ProcessExecEvent {
    pub timestamp: SystemTime,
    pub pid: u32,
    pub ppid: u32,
    pub uid:String,
    pub user: Option<String>,
    pub executable_path: String,
    pub command_line: Vec<String>,
}

#[derive(Debug)]
pub struct ProcessExitEvent {
    pub timestamp: SystemTime,
    pub pid: u32,
    pub exit_code: i32,
}


pub enum ProcessEvent {
    Start(ProcessExecEvent),
    Exit(ProcessExitEvent),
}

pub trait ProcessProvider {
    fn start(&self) -> anyhow::Result<()>;
}