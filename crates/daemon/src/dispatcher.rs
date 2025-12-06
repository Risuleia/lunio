// use std::sync::{Arc, Mutex};

// use lunio_engine::jobs::event::JobEvent;
// use lunio_protocol::{Envelope, Event, Topic};
// use tokio::sync::mpsc::Receiver;

// use crate::registry::Registry;

// pub struct DaemonEvent {
//     pub topic: Topic,
//     pub event: Envelope<Event>
// }

// pub async fn dispatcher_loop(
//     mut rx: Receiver<DaemonEvent>,
//     registry: Arc<Mutex<Registry>>
// ) {
//     while let Some(msg) = rx.recv().await {
//         let registry = registry.lock().unwrap();

//         for client in registry.clients.values() {
//             if client.subs.contains(&msg.topic) {
//                 let _ = client.tx.try_send(msg.event.clone());
//             }
//         }
//     }
// }