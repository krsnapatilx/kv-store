use std::path::PathBuf;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct StoreConfig {
    pub data_dir: PathBuf,
    pub segment_size: u64,
    pub fsync_policy: FsyncPolicy,
    pub compaction_threshold: usize,
}

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum FsyncPolicy {
    Always,
    Never,
    Batch,
}

impl StoreConfig {
    pub fn new(data_dir: &std::path::Path) -> Self {
        Self { 
            data_dir: data_dir.to_path_buf(), 
            segment_size: 1024 * 1024, 
            fsync_policy: FsyncPolicy::Always,
            compaction_threshold: 3
        }
    }
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self { 
            data_dir: PathBuf::from("data"), 
            segment_size: 1024 * 1024, 
            fsync_policy: FsyncPolicy::Always, 
            compaction_threshold: 3 
        }
    }
}