use crate::jobs::job::JobId;
use std::time::{SystemTime, UNIX_EPOCH};

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Fired by scheduler and workers
#[derive(Debug, Clone)]
pub enum JobEvent {

    Queued {
        id: JobId,
        at: u64,
    },

    Started {
        id: JobId,
        at: u64,
        attempt: u8,
    },

    Progress {
        id: JobId,
        at: u64,
        done: u64,
        total: u64,
    },

    Retry {
        id: JobId,
        at: u64,
        attempt: u8,
        delay_ms: u64,
        reason: String,
    },

    Completed {
        id: JobId,
        at: u64,
    },

    Failed {
        id: JobId,
        at: u64,
        reason: String,
    },

    Cancelled {
        id: JobId,
        at: u64,
    },
}

impl JobEvent {

    pub fn queued(id: JobId) -> Self {
        Self::Queued { id, at: now_unix() }
    }

    pub fn started(id: JobId, attempt: u8) -> Self {
        Self::Started { id, at: now_unix(), attempt }
    }

    pub fn progress(id: JobId, done: u64, total: u64) -> Self {
        Self::Progress { id, at: now_unix(), done, total }
    }

    pub fn retry(id: JobId, attempt: u8, delay_ms: u64, reason: String) -> Self {
        Self::Retry {
            id,
            at: now_unix(),
            attempt,
            delay_ms,
            reason,
        }
    }

    pub fn completed(id: JobId) -> Self {
        Self::Completed { id, at: now_unix() }
    }

    pub fn failed(id: JobId, reason: String) -> Self {
        Self::Failed { id, at: now_unix(), reason }
    }

    pub fn cancelled(id: JobId) -> Self {
        Self::Cancelled { id, at: now_unix() }
    }

    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            JobEvent::Completed { .. }
                | JobEvent::Failed { .. }
                | JobEvent::Cancelled { .. }
        )
    }

    pub fn id(&self) -> JobId {
        match self {
            JobEvent::Queued { id, .. }
            | JobEvent::Started { id, .. }
            | JobEvent::Progress { id, .. }
            | JobEvent::Retry { id, .. }
            | JobEvent::Completed { id, .. }
            | JobEvent::Failed { id, .. }
            | JobEvent::Cancelled { id, .. } => *id,
        }
    }
}