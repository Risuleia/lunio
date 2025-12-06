use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::PROTOCOL_VERSION;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Envelope<T> {
    pub protocol: u16,
    pub message_id: Uuid,
    pub session_id: Uuid,
    pub payload: T
}

impl<T> Envelope<T> {
    pub fn new(session_id: Uuid, payload: T) -> Self {
        Self {
            protocol: PROTOCOL_VERSION,
            message_id: Uuid::new_v4(),
            session_id,
            payload
        }
    }
}