use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use bincode::{config::standard, decode_from_slice, encode_to_vec};
use tokio::fs;

use crate::jobs::{
    event::JobEvent,
    job::{JobId, JobSpec},
    persist::PersistentJob,
    state::{JobState, JobStatus},
};

pub struct JobStore {
    root: PathBuf,

    /// Authoritative runtime state
    pub jobs: Arc<RwLock<HashMap<JobId, (JobSpec, JobState)>>>,
}

impl JobStore {

    pub async fn open(dir: impl AsRef<Path>) -> Self {
        let root = dir.as_ref().to_path_buf();
        let jobs_dir = root.join("jobs");

        fs::create_dir_all(&jobs_dir).await.unwrap();

        let store = Self {
            root,
            jobs: Arc::new(RwLock::new(HashMap::new())),
        };

        store.load_all().await;
        store
    }

    /* ==========================
       READ API
    ========================== */

    pub fn snapshot(&self) -> Vec<(JobId, JobSpec, JobState)> {
        self.jobs
            .read()
            .unwrap()
            .iter()
            .map(|(id, (spec, state))| (*id, spec.clone(), state.clone()))
            .collect()
    }

    pub fn get(&self, id: JobId) -> Option<JobState> {
        self.jobs.read().unwrap().get(&id).map(|(_, s)| s.clone())
    }

    /* ==========================
       WRITE API
    ========================== */

    pub fn insert_new(&self, spec: JobSpec) {

        let id = spec.id;
        let state = JobState::new();

        {
            let mut map = self.jobs.write().unwrap();
            map.insert(id, (spec.clone(), state.clone()));
        }

        let _ = self.save_job(id, spec, state);
    }

    pub fn on_event(&self, evt: &JobEvent) {

        let mut map = self.jobs.write().unwrap();

        match evt {

            JobEvent::Queued { id, .. } => {
                map.entry(*id)
                    .and_modify(|(_, st)| *st = JobState::new())
                    .or_insert_with(|| panic!("Queued before JobSpec insert"));
            }

            JobEvent::Started { id, .. } => {
                if let Some((_, st)) = map.get_mut(id) {
                    st.start();
                }
            }

            JobEvent::Retry { id, .. } => {
                if let Some((_, st)) = map.get_mut(id) {
                    st.reset_for_retry();
                }
            }

            JobEvent::Completed { id, .. } => {
                if let Some((_, st)) = map.get_mut(id) {
                    st.complete();
                }
            }

            JobEvent::Failed { id, reason, .. } => {
                if let Some((_, st)) = map.get_mut(id) {
                    st.fail(reason.clone());
                }
            }

            JobEvent::Cancelled { id, .. } => {
                if let Some((_, st)) = map.get_mut(id) {
                    st.cancel();
                }
            }

            JobEvent::Progress { id, done, total, .. } => {
                if let Some((_, st)) = map.get_mut(id) {
                    st.set_progress(*done, *total);
                }
            }
        }

        // Persist immediately
        if let Some((spec, state)) = map.get(&evt.id()) {
            let _ = self.save_job(evt.id(), spec.clone(), state.clone());
        }
    }

    /* ==========================
       RECOVERY
    ========================== */

    pub async fn reconcile<F>(&self, mut enqueue: F)
    where
        F: FnMut(JobSpec),
    {
        let snapshot = self.snapshot();

        for (_id, spec, state) in snapshot {

            match &state.status {

                JobStatus::Queued => {
                    enqueue(spec);
                }

                JobStatus::Running => {
                    // crash-recovery — mark failed
                    let mut crashed = state.clone();
                    crashed.fail("crashed (daemon restart)".into());
                    let _ = self.save_job(spec.id, spec.clone(), crashed);
                }

                JobStatus::Failed { .. } => {
                    if state.attempts < spec.retry.max_retries {
                        enqueue(spec);
                    }
                }

                _ => {}
            }
        }
    }

    /* ==========================
       DISK IO
    ========================== */

    fn job_path(&self, id: JobId) -> PathBuf {
        self.root.join("jobs").join(format!("{}.bin", id.0))
    }

    fn save_job(&self, id: JobId, spec: JobSpec, state: JobState) -> std::io::Result<()> {

        let path = self.job_path(id);
        let tmp = path.with_extension("tmp");

        let payload = encode_to_vec(PersistentJob::new(spec, state), standard()).unwrap();

        std::fs::write(&tmp, payload)?;
        std::fs::rename(tmp, path)?;

        Ok(())
    }

    async fn load_all(&self) {

        let job_dir = self.root.join("jobs");

        let mut rd = match fs::read_dir(&job_dir).await {
            Ok(r) => r,
            Err(_) => return,
        };

        while let Ok(Some(entry)) = rd.next_entry().await {
            if let Ok(bytes) = fs::read(entry.path()).await {

                if let Ok((job, _)) = decode_from_slice::<PersistentJob, _>(&bytes, standard()) {
                    if let Ok(job) = job.validate() {
                        self.jobs
                            .write()
                            .unwrap()
                            .insert(job.spec.id, (job.spec, job.state));
                    }
                }
            }
        }
    }
}