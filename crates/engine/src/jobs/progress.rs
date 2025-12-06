#[derive(Debug, Clone)]
pub enum JobStatus {
    Pending,
    Running,
    Completed,
    Failed(String),
    Cancelled,
}

#[derive(Debug, Clone)]
pub struct JobProgress {
    pub done: u64,
    pub total: u64,
    pub status: JobStatus,
}
