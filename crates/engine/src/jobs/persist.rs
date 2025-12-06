use bincode::{Encode, Decode};
use std::time::{SystemTime, UNIX_EPOCH};
use crate::jobs::{job::JobSpec, state::JobState};

/// Increment this on breaking schema changes
const PERSIST_VERSION: u16 = 1;

/// Persistent job record format
#[derive(Debug, Clone, Encode, Decode)]
pub struct PersistentJob {

    /// Schema version
    pub version: u16,

    /// When persisted to disk
    pub saved_at_unix: u64,

    /// Forward-compatibility / padding
    pub reserved: [u8; 8],

    /// Job spec
    pub spec: JobSpec,

    /// Job state
    pub state: JobState,
}

impl PersistentJob {

    pub fn new(spec: JobSpec, state: JobState) -> Self {
        Self {
            version: PERSIST_VERSION,
            saved_at_unix: now_unix(),
            reserved: [0; 8],
            spec,
            state,
        }
    }

    /// Defensive decoding boundary
    pub fn validate(self) -> Result<Self, String> {

        if self.version > PERSIST_VERSION {
            return Err(format!("unsupported persistent version: {}", self.version));
        }

        Ok(self)
    }
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
