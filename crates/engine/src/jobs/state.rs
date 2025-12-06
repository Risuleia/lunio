use bincode::{Encode, Decode};
use std::time::{SystemTime, UNIX_EPOCH};
use super::job::JobId;

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum JobStatus {
    Queued,
    WaitingDependencies {
        unresolved: Vec<JobId>,
    },
    Running,
    Completed,
    Failed { reason: String },
    Cancelled,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct JobState {
    pub status: JobStatus,

    // ✅ Persisted timeline (portable)
    pub created_at: u64,
    pub started_at: Option<u64>,
    pub finished_at: Option<u64>,
    pub last_progress: Option<u64>,

    pub attempts: u8,

    pub done: u64,
    pub total: u64,
}

impl JobState {
    pub fn new() -> Self {
        let t = now_unix();
        Self {
            status: JobStatus::Queued,
            created_at: t,
            started_at: None,
            finished_at: None,
            last_progress: None,
            attempts: 0,
            done: 0,
            total: 0,
        }
    }

    pub fn start(&mut self) {
        if !matches!(self.status, JobStatus::Queued | JobStatus::WaitingDependencies { .. }) {
            return;
        }

        self.status = JobStatus::Running;
        self.started_at = Some(now_unix());
        self.attempts += 1;
    }

    pub fn complete(&mut self) {
        if !matches!(self.status, JobStatus::Running) {
            return;
        }

        self.status = JobStatus::Completed;
        self.finished_at = Some(now_unix());
    }

    pub fn fail(&mut self, reason: String) {
        if !matches!(self.status, JobStatus::Running) {
            return;
        }

        self.status = JobStatus::Failed { reason };
        self.finished_at = Some(now_unix());
    }

    pub fn cancel(&mut self) {
        self.status = JobStatus::Cancelled;
        self.finished_at = Some(now_unix());
    }

    pub fn mark_waiting(&mut self, deps: Vec<JobId>) {
        if !matches!(self.status, JobStatus::Queued | JobStatus::Running) {
            return;
        }

        self.status = JobStatus::WaitingDependencies {
            unresolved: deps,
        };
    }

    pub fn set_progress(&mut self, done: u64, total: u64) {
        if !matches!(self.status, JobStatus::Running) {
            return;
        }

        self.done = done;
        self.total = total;
        self.last_progress = Some(now_unix());
    }

    

    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            JobStatus::Completed |
            JobStatus::Cancelled |
            JobStatus::Failed { .. }
        )
    }

    pub fn is_running(&self) -> bool {
        matches!(self.status, JobStatus::Running)
    }

    pub fn can_retry(&self) -> bool {
        matches!(self.status, JobStatus::Failed { .. })
    }

    pub fn is_waiting(&self) -> bool {
        matches!(self.status, JobStatus::WaitingDependencies { .. })
    }

    pub fn is_active(&self) -> bool {
        matches!(
            self.status,
            JobStatus::Queued |
            JobStatus::Running |
            JobStatus::WaitingDependencies { .. }
        )
    }

    pub fn reset_for_retry(&mut self) {
        self.status = JobStatus::Queued;
        self.started_at = None;
        self.finished_at = None;
        self.last_progress = None;
        self.done = 0;
        self.total = 0;
    }

    pub fn is_done(&self) -> bool {
        matches!(
            self.status,
            JobStatus::Completed | JobStatus::Cancelled | JobStatus::Failed { .. }
        )
    }
}
