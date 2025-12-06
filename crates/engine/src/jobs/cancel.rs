use std::collections::HashSet;
use std::sync::{Mutex};

use super::job::JobId;

/// Central cancellation map
pub struct CancelRegistry {
    cancelled: Mutex<HashSet<JobId>>,
}

impl CancelRegistry {
    pub fn new() -> Self {
        Self {
            cancelled: Mutex::new(HashSet::new()),
        }
    }

    /// Mark job as cancelled
    pub fn cancel(&self, id: JobId) {
        self.cancelled.lock().unwrap().insert(id);
    }

    /// Register job (optional)
    pub fn register(&self, _id: JobId) {
        // reserved for future expansion
    }

    /// Remove job from cancel map
    pub fn unregister(&self, id: JobId) {
        self.cancelled.lock().unwrap().remove(&id);
    }

    /// Check if job is cancelled
    pub fn is_cancelled(&self, id: JobId) -> bool {
        self.cancelled.lock().unwrap().contains(&id)
    }
}
