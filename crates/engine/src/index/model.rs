use std::path::PathBuf;
use bincode::{BorrowDecode, Decode, Encode, de::{BorrowDecoder, Decoder}, enc::Encoder, error::{DecodeError, EncodeError}};
use uuid::Uuid;

/* ================================
   FILE IDENTIFIER
================================ */

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct FileId(pub Uuid);

impl Encode for FileId {
    fn encode<E: Encoder>(&self, encoder: &mut E) -> Result<(), EncodeError> {
        self.0.into_bytes().encode(encoder)
    }
}

impl<Context> Decode<Context> for FileId {
    fn decode<D: Decoder<Context = Context>>(decoder: &mut D) -> Result<Self, DecodeError> {
        let bytes = <[u8; 16]>::decode(decoder)?;
        Ok(FileId(Uuid::from_bytes(bytes)))
    }
}

impl<'de, Context> BorrowDecode<'de, Context> for FileId {
    fn borrow_decode<D: BorrowDecoder<'de, Context = Context>>(
        decoder: &mut D,
    ) -> Result<Self, DecodeError> {
        let bytes = <[u8; 16]>::borrow_decode(decoder)?;
        Ok(FileId(Uuid::from_bytes(bytes)))
    }
}

impl FileId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

/* ================================
   FILE RECORD (CORE INDEX ENTITY)
================================ */

#[derive(Debug, Clone, Encode, Decode)]
pub struct FileRecord {

    /* -------- Identity -------- */

    pub id: FileId,

    /// Absolute path (canonical)
    pub path: PathBuf,

    /// Parent directory path (denormalized)
    pub parent: PathBuf,

    /* -------- Naming -------- */

    /// Filename only
    pub name: String,

    /// Extension (lowercase, no dot)
    pub ext: Option<String>,

    /* -------- Type -------- */

    pub is_dir: bool,
    pub is_symlink: bool,

    /* -------- Metadata -------- */

    /// File size in bytes (None for folders & symlinks)
    pub size: u64,

    /// Unix timestamp (seconds)
    pub modified_unix: u64,

    /* -------- Versioning -------- */

    /// Incremented each time record is replaced
    pub generation: u64,
}
