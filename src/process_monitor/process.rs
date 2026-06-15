pub struct ProcessEvent {
    pub timestamp: String,
    pub pid: u32,
    pub parent_pid: u32,
    pub user: String,
    pub executable_path: String,
    pub command_line: String,
}

pub trait ProcessProvider {
    fn start(&self) -> anyhow::Result<()>;
}