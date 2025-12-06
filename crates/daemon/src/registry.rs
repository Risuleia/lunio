use std::collections::{HashMap, HashSet};

use lunio_protocol::{Envelope, Event, Topic};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub type SessionId = Uuid;

/* ===========================
   CLIENT STATE
=========================== */

pub struct ClientState {
    pub tx: Sender<Envelope<Event>>,
}

/* ===========================
   REGISTRY
=========================== */

pub struct Registry {
    clients: HashMap<SessionId, ClientState>,
    topics: HashMap<Topic, HashSet<SessionId>>,
}

impl Registry {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            topics: HashMap::new(),
        }
    }

    /* ===========================
       CLIENT LIFECYCLE
    ============================ */

    pub fn register(&mut self, id: SessionId, tx: Sender<Envelope<Event>>) {
        self.clients.insert(id, ClientState { tx });
    }

    pub fn remove(&mut self, id: &SessionId) {
        self.clients.remove(id);
        self.unsubscribe_all(id);
    }

    /* ===========================
       SUBSCRIPTIONS
    ============================ */

    pub fn subscribe(&mut self, id: &SessionId, topic: Topic) {
        self.topics
            .entry(topic)
            .or_default()
            .insert(*id);
    }

    pub fn unsubscribe(&mut self, id: &SessionId, topic: &Topic) {
        if let Some(subs) = self.topics.get_mut(topic) {
            subs.remove(id);
        }
    }

    pub fn unsubscribe_all(&mut self, id: &SessionId) {
        for subs in self.topics.values_mut() {
            subs.remove(id);
        }
    }

    /* ===========================
       MESSAGE DELIVERY
    ============================ */

    /// Broadcast to all subscribers of a topic
    pub async fn broadcast(&self, topic: Topic, evt: Event) {
        let Some(subs) = self.topics.get(&topic) else { return };

        for id in subs {
            if let Some(client) = self.clients.get(id) {
                let _ = client.tx.send(Envelope::new(*id, evt.clone())).await;
            }
        }
    }

    /// Send directly to a client
    pub async fn send_to(&self, id: &SessionId, evt: Event) {
        if let Some(c) = self.clients.get(id) {
            let _ = c.tx.send(Envelope::new(*id, evt)).await;
        }
    }

    /* ===========================
       DEBUG / VISIBILITY HELPERS
    ============================ */

    pub fn is_connected(&self, id: &SessionId) -> bool {
        self.clients.contains_key(id)
    }

    pub fn subscriber_count(&self, topic: &Topic) -> usize {
        self.topics.get(topic).map(|s| s.len()).unwrap_or(0)
    }

    pub fn client_count(&self) -> usize {
        self.clients.len()
    }
}
