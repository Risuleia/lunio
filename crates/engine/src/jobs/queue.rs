use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    time::{Duration, Instant},
};
use crate::jobs::{
    job::{JobId, JobSpec, Priority},
    state::{JobState, JobStatus},
};

#[derive(Clone)]
struct QueuedJob {
    priority: Priority,
    ready_at: Instant,
    enqueued_at: Instant,
    id: JobId,
}

impl Ord for QueuedJob {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority
            .cmp(&other.priority)
            .then_with(|| other.ready_at.cmp(&self.ready_at))
            .then_with(|| other.enqueued_at.cmp(&self.enqueued_at))
    }
}

impl PartialOrd for QueuedJob {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for QueuedJob {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for QueuedJob {}

pub struct JobQueue {

    pub jobs: HashMap<JobId, JobSpec>,
    states: HashMap<JobId, JobState>,

    ready: BinaryHeap<QueuedJob>,
    scheduled: HashSet<JobId>,  // ✅ prevents duplicates

    waiting: HashMap<JobId, HashSet<JobId>>,
    dependents: HashMap<JobId, HashSet<JobId>>,
}

impl JobQueue {

    pub fn new() -> Self {
        Self {
            jobs: HashMap::new(),
            states: HashMap::new(),

            ready: BinaryHeap::new(),
            scheduled: HashSet::new(),

            waiting: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    pub fn push(&mut self, spec: JobSpec) {

        let id = spec.id;
        let deps: HashSet<JobId> = spec.dependencies.iter().cloned().collect();

        self.states.insert(id, JobState::new());
        self.jobs.insert(id, spec.clone());

        for d in &deps {
            self.dependents.entry(*d).or_default().insert(id);
        }

        if !deps.is_empty() {
            self.waiting.insert(id, deps);
            self.states
                .get_mut(&id)
                .unwrap()
                .mark_waiting(spec.dependencies.clone());
            return;
        }

        self.enqueue_ready(id, Instant::now());
    }

    pub fn on_dependency_completed(&mut self, dep: JobId) {
        if let Some(children) = self.dependents.remove(&dep) {
            for job in children {
                if let Some(unmet) = self.waiting.get_mut(&job) {
                    unmet.remove(&dep);
                    if unmet.is_empty() {
                        self.waiting.remove(&job);
                        self.enqueue_ready(job, Instant::now());
                    }
                }
            }
        }
    }

    fn on_dependency_failed(&mut self, dep: JobId) {
        if let Some(children) = self.dependents.remove(&dep) {
            for job in children {
                if let Some(st) = self.states.get_mut(&job) {
                    st.fail(format!("dependency {} failed/cancelled", dep.0));
                }
                self.waiting.remove(&job);
            }
        }
    }

    pub fn schedule_retry(&mut self, id: JobId, delay_ms: u64) {

        if let Some(st) = self.states.get_mut(&id) {
            st.reset_for_retry();
        }

        let at = Instant::now() + Duration::from_millis(delay_ms);
        self.enqueue_ready(id, at);
    }

    fn enqueue_ready(&mut self, id: JobId, ready_at: Instant) {

        if !self.scheduled.insert(id) {
            return;
        }

        let spec = self.jobs.get(&id).expect("unknown job");

        self.ready.push(QueuedJob {
            priority: spec.priority,
            ready_at,
            enqueued_at: Instant::now(),
            id,
        });

        if let Some(st) = self.states.get_mut(&id) {
            if matches!(st.status, JobStatus::WaitingDependencies { .. }) {
                st.status = JobStatus::Queued;
            }
        }
    }

    pub fn pop_ready(&mut self) -> Option<JobSpec> {
        let now = Instant::now();

        while let Some(top) = self.ready.peek() {

            if top.ready_at > now {
                return None;
            }

            let q = self.ready.pop().unwrap();
            self.scheduled.remove(&q.id);

            match self.states.get(&q.id).map(|s| &s.status) {
                Some(JobStatus::Cancelled) | Some(JobStatus::Failed { .. }) => continue,
                _ => {}
            }

            return self.jobs.get(&q.id).cloned();
        }

        None
    }

    pub fn mark_running(&mut self, id: JobId) {
        if let Some(st) = self.states.get_mut(&id) {
            st.start();
        }
    }

    pub fn mark_completed(&mut self, id: JobId) {
        if let Some(st) = self.states.get_mut(&id) {
            st.complete();
        }
        self.on_dependency_completed(id);
    }

    pub fn mark_failed(&mut self, id: JobId, reason: String) {
        if let Some(st) = self.states.get_mut(&id) {
            st.fail(reason);
        }
        self.on_dependency_failed(id);
    }

    pub fn cancel(&mut self, id: JobId) {
        if let Some(st) = self.states.get_mut(&id) {
            st.cancel();
        }
        self.on_dependency_failed(id);
    }

    pub fn state(&self, id: &JobId) -> Option<&JobState> {
        self.states.get(id)
    }

    pub fn all_states(&self) -> impl Iterator<Item = (&JobId, &JobState)> {
        self.states.iter()
    }
}
