use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Accepted,
    Completed,
    Rejected { reason: String },
}
