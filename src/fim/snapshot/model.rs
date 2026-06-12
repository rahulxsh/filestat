use std::path::PathBuf;

pub struct SnapshotDiff {
    pub added: Vec<PathBuf>,
    pub deleted: Vec<PathBuf>,
    pub modified: Vec<PathBuf>,
}