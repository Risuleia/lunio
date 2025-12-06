use std::{sync::Arc, time::Duration};
use tokio::sync::{Mutex, mpsc};

use crate::jobs::{
    cancel::CancelRegistry,
    event::JobEvent,
    job::{JobId, JobSpec},
    queue::JobQueue,
    store::JobStore,
    worker::WorkerMsg,
};

pub enum WorkerResult {
    Success(JobId),
    Failed(JobId, String),
    Cancelled(JobId),
}

pub enum SchedulerCommand {
    Submit(JobSpec),
    Cancel(JobId),
}

pub struct Scheduler {
    queue: Arc<Mutex<JobQueue>>,
    cancel: Arc<CancelRegistry>,
    store: JobStore,

    cmd_rx: mpsc::Receiver<SchedulerCommand>,
    event_tx: mpsc::Sender<JobEvent>,
    worker_tx: mpsc::Sender<JobSpec>,
    result_rx: mpsc::Receiver<WorkerMsg>,
}

impl Scheduler {

    pub async fn start(
        worker_count: usize,
        data_dir: impl AsRef<std::path::Path>,
        event_tx: mpsc::Sender<JobEvent>,
    ) -> mpsc::Sender<SchedulerCommand> {

        let store = JobStore::open(data_dir).await;

        let (cmd_tx, cmd_rx) = mpsc::channel(256);
        let (worker_tx, worker_rx) = mpsc::channel(256);
        let (result_tx, result_rx) = mpsc::channel(256);

        let queue = Arc::new(Mutex::new(JobQueue::new()));
        let cancel = Arc::new(CancelRegistry::new());

        let shared_rx = Arc::new(Mutex::new(worker_rx));

        for _ in 0..worker_count {
            crate::jobs::worker::start_worker(
                shared_rx.clone(),
                result_tx.clone(),
                cancel.clone()
            );
        }

        let mut sched = Self {
            queue,
            cancel,
            store,
            cmd_rx,
            event_tx,
            worker_tx,
            result_rx,
        };

        sched.boot().await;

        tokio::spawn(async move {
            sched.run().await;
        });

        cmd_tx
    }

    /// Restore incomplete jobs from storage
    async fn boot(&mut self) {
        let queue = self.queue.clone();

        self.store
            .reconcile(|spec| {
                let queue = queue.clone();
                tokio::spawn(async move {
                    queue.lock().await.push(spec);
                });
            })
            .await;
    }

    /* ====================
       MAIN LOOP
       ==================== */

    async fn run(&mut self) {
        loop {
            tokio::select! {

                Some(cmd) = self.cmd_rx.recv() =>
                    self.handle_command(cmd).await,

                Some(msg) = self.result_rx.recv() =>
                    self.handle_worker(msg).await,

                _ = tokio::time::sleep(Duration::from_millis(50)) =>
                    self.dispatch_ready().await,
            }
        }
    }

    /* ====================
       WORKER EVENTS
       ==================== */

    async fn handle_worker(&mut self, msg: WorkerMsg) {
        match msg {

            WorkerMsg::Progress(id, done, total) => {
                let evt = JobEvent::progress(id, done, total);
                self.store.on_event(&evt);
                let _ = self.event_tx.send(evt).await;
            }

            WorkerMsg::Result(res) =>
                self.handle_result(res).await,
        }
    }

    /* ====================
       CLIENT COMMANDS
       ==================== */

    async fn handle_command(&mut self, cmd: SchedulerCommand) {
        match cmd {

            SchedulerCommand::Submit(spec) => {
                self.queue.lock().await.push(spec.clone());
                self.store.insert_new(spec.clone());

                let evt = JobEvent::queued(spec.id);
                self.store.on_event(&evt);
                let _ = self.event_tx.send(evt).await;
            }

            SchedulerCommand::Cancel(id) => {

                // ✅ Persist first
                let evt = JobEvent::cancelled(id);
                self.store.on_event(&evt);
                let _ = self.event_tx.send(evt).await;

                // ✅ Then notify runtime
                self.cancel.cancel(id);
                self.queue.lock().await.cancel(id);
            }
        }
    }

    /* ====================
       WORKER RESULT
       ==================== */

    async fn handle_result(&mut self, result: WorkerResult) {
        match result {

            WorkerResult::Success(id) => {
                self.queue.lock().await.mark_completed(id);

                let evt = JobEvent::completed(id);
                self.store.on_event(&evt);
                let _ = self.event_tx.send(evt).await;
            }

            WorkerResult::Cancelled(id) => {
                self.queue.lock().await.cancel(id);

                let evt = JobEvent::cancelled(id);
                self.store.on_event(&evt);
                let _ = self.event_tx.send(evt).await;
            }

            WorkerResult::Failed(id, reason) => {

                let (retry, attempt, delay) = {
                    let mut q = self.queue.lock().await;

                    let spec = q.jobs.get(&id).cloned();
                    let state = q.state(&id).cloned();

                    let retry =
                        matches!(
                            (state.as_ref(), spec.as_ref()),
                            (Some(s), Some(p)) if s.attempts < p.retry.max_retries
                        );

                    let delay = spec.map(|p| p.retry.delay_ms).unwrap_or(0);
                    let attempt = state.map(|s| s.attempts).unwrap_or(1);

                    if retry {
                        q.schedule_retry(id, delay);
                    } else {
                        q.mark_failed(id, reason.clone());
                    }

                    (retry, attempt, delay)
                };

                let evt = if retry {
                    JobEvent::retry(id, attempt, delay, reason.clone())
                } else {
                    JobEvent::failed(id, reason)
                };

                self.store.on_event(&evt);
                let _ = self.event_tx.send(evt).await;
            }
        }
    }

    /* ====================
       DISPATCH LOOP
       ==================== */

    async fn dispatch_ready(&mut self) {
        loop {
            let job = { self.queue.lock().await.pop_ready() };
            let job = match job {
                Some(j) => j,
                None => break,
            };

            let id = job.id;

            // ✅ Send first, then mark
            if self.worker_tx.send(job).await.is_ok() {

                let attempt = {
                    let state = self.queue.lock().await.state(&id).cloned();
                    state.map(|s| s.attempts).unwrap_or(1)
                };

                let evt = JobEvent::started(id, attempt);
                self.store.on_event(&evt);
                let _ = self.event_tx.send(evt).await;

            } else {
                // Worker pool unavailable → retry later
                self.queue.lock().await.schedule_retry(id, 1000);
            }
        }
    }
}
