use std::collections::{HashSet, BinaryHeap};
use crate::thumbs::model::{ThumbSpec};

#[derive(Eq)]
struct JobWrap(ThumbSpec);

impl Ord for JobWrap {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.priority.cmp(&other.0.priority)
    }
}
impl PartialOrd for JobWrap {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for JobWrap {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

pub struct ThumbScheduler {
    queue: BinaryHeap<JobWrap>,
    active: HashSet<String>,
}

impl ThumbScheduler {
    pub fn new() -> Self {
        Self {
            queue: BinaryHeap::new(),
            active: HashSet::new(),
        }
    }

    pub fn push(&mut self, job: ThumbSpec) {
        self.queue.push(JobWrap(job));
    }

    pub fn pop(&mut self) -> Option<ThumbSpec> {
        self.queue.pop().map(|w| w.0)
    }
}
