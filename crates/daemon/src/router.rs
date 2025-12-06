use std::sync::Arc;

use lunio_engine::jobs::{job::{JobKind, JobSpec}, scheduler::SchedulerCommand};
use lunio_protocol::{Command, Event};
use tokio::sync::{Mutex, mpsc::{self}};

use crate::registry::{Registry, SessionId};

pub async fn handle_command(
    session: SessionId,
    cmd: Command,
    registry: Arc<Mutex<Registry>>,
    jobs: mpsc::Sender<SchedulerCommand>
) {
    match cmd {

        /* ======================
           LIFECYCLE
        ====================== */

        Command::Hello { .. } => {
            let mut r = registry.lock().await;
            r.send_to(&session, Event::Welcome {
                session_id: session,
                server_version: "dev".into(),
                server_capabilities: vec![],
            }).await;
        }

        Command::Disconnect => {
            registry.lock().await.remove(&session);
        }

        /* ======================
           SUBSCRIPTIONS
        ====================== */

        Command::Subscribe { topics } => {
            let mut r = registry.lock().await;
            for t in topics {
                r.subscribe(&session, t);
            }
        }

        /* ======================
           JOB CONTROL
        ====================== */

        Command::ListJobs => {
            // TODO: fetch from job store later
        }

        /* ======================
           FILESYSTEM JOBS
        ====================== */

        Command::Delete { path } => {
            let job = JobSpec::new(
                JobKind::DeleteTree { target: path }
            );
            let _ = jobs.send(SchedulerCommand::Submit(job)).await;
        }

        Command::Copy { from, to } => {
            let job = JobSpec::new(JobKind::Copy { from, to });
            let _ = jobs.send(SchedulerCommand::Submit(job)).await;
        }

        Command::Move { from, to } => {
            let job = JobSpec::new(JobKind::Move { from, to });
            let _ = jobs.send(SchedulerCommand::Submit(job)).await;
        }

        /* ======================
           INDEX
        ====================== */

        Command::OpenFolder { path } => {
            // kick off index scan job
            let job = JobSpec::new(JobKind::IndexScan { root: path });
            let _ = jobs.send(SchedulerCommand::Submit(job)).await;
        }

        /* ======================
           SEARCH & BROWSE
        ====================== */

        Command::Search { query } => {
            // TODO: call index search, emit Event::SearchResults
        }

        Command::Browse { path } => {
            // TODO: index list
        }

        /* ======================
           THUMBNAILS
        ====================== */

        Command::RequestThumbnail { file_id } => {
            // TODO: submit to thumbnail service
        }

        _ => {}
    }
}