use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;
use bincode::{BorrowDecode, Decode, Encode, de::{BorrowDecoder, Decoder}, enc::Encoder, error::{DecodeError, EncodeError}};

use crate::jobs::JOB_VERSION;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct JobId(pub Uuid);

// ---- bincode v2 adapter for Uuid ----
impl Encode for JobId {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.into_bytes().encode(encoder)
    }
}

impl<Context> Decode<Context> for JobId {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let bytes = <[u8; 16]>::decode(decoder)?;
        Ok(JobId(Uuid::from_bytes(bytes)))
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for JobId {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let bytes = <[u8; 16]>::borrow_decode(decoder)?;
        Ok(JobId(Uuid::from_bytes(bytes)))
    }
}

impl JobId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Encode, Decode)]
pub enum Priority {
    Low,
    Normal,
    High,
    Critical,
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct RetryPolicy {
    pub max_retries: u8,
    pub delay_ms: u64,
}

impl RetryPolicy {
    pub fn never() -> Self {
        Self { max_retries: 0, delay_ms: 0 }
    }

    pub fn standard() -> Self {
        Self { max_retries: 3, delay_ms: 500 }
    }
}

#[derive(Debug, Clone, Encode, Decode)]
pub enum JobKind {
    Copy { from: String, to: String },
    Move { from: String, to: String },
    DeleteTree { target: String },
    IndexScan { root: String },
    RebuildIndex,
    Thumbnail { file: String },
}

#[derive(Debug, Clone, Encode, Decode)]
pub struct JobSpec {
    pub version: u16,
    pub id: JobId,
    pub kind: JobKind,

    pub priority: Priority,
    pub retry: RetryPolicy,

    pub dependencies: Vec<JobId>,

    pub created_at_unix: u64,
}

impl JobSpec {
    pub fn new(kind: JobKind) -> Self {
        Self {
            version: JOB_VERSION,
            id: JobId::new(),
            kind,
            priority: Priority::Normal,
            retry: RetryPolicy::standard(),
            dependencies: Vec::new(),
            created_at_unix: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    pub fn with_priority(mut self, p: Priority) -> Self {
        self.priority = p;
        self
    }

    pub fn with_retry(mut self, r: RetryPolicy) -> Self {
        self.retry = r;
        self
    }

    pub fn depends_on(mut self, id: JobId) -> Self {
        self.dependencies.push(id);
        self
    }
}
