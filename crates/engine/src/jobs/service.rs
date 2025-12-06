// use tokio::sync::mpsc;
// use std::sync::{Arc, Mutex};
// use uuid::Uuid;

// use super::store::JobStore;
// use super::progress::*;
// use crate::jobs::job::JobId;

// pub struct JobService {
//     store: Arc<Mutex<JobStore>>,
//     events: mpsc::Sender<(JobId, JobProgress)>,
// }

// impl JobService {
//     pub fn start(
//         events: mpsc::Sender<(JobId, JobProgress)>
//     ) -> Arc<Self> {
//         Arc::new(Self {
//             store: Arc::new(Mutex::new(JobStore::new())),
//             events,
//         })
//     }

//     pub fn spawn_copy(&self, from: String, to: String) -> JobId {
//         let id = JobId(Uuid::new_v4());
//         let store = self.store.clone();
//         let tx = self.events.clone();

//         {
//             let mut s = store.lock().unwrap();
//             s.insert(id, 100); // placeholder
//         }

//         tokio::spawn(async move {
//             // TODO: implement real copy later
//             for i in 0..100 {
//                 tokio::time::sleep(std::time::Duration::from_millis(50)).await;

//                 let prog = {
//                     let mut s = store.lock().unwrap();
//                     s.update(id, i + 1);
//                     s.jobs.get(&id).cloned().unwrap()
//                 };

//                 let _ = tx.send((id, prog)).await;
//             }

//             store.lock().unwrap().complete(id);
//         });

//         id
//     }
// }