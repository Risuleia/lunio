use std::path::{Path, PathBuf};
use std::fs::{self, File, OpenOptions};
use std::io::{Write, Read};

use bincode::{Decode, Encode};
use bincode::{encode_to_vec, decode_from_slice, config::standard};

use crate::index::store::IndexStore;

pub struct IndexPersistence {
    root: PathBuf,
}

impl IndexPersistence {
    pub fn new(root: impl AsRef<Path>) -> Self {
        Self {
            root: root.as_ref().to_path_buf(),
        }
    }

    fn snapshot_path(&self) -> PathBuf {
        self.root.join("index.snapshot")
    }

    fn journal_path(&self) -> PathBuf {
        self.root.join("index.journal")
    }

    /* =====================
       SNAPSHOT
    ===================== */

    pub fn save_snapshot(&self, store: &IndexStore) -> std::io::Result<()> {
        let blob = encode_to_vec(store, standard()).unwrap();
        fs::write(self.snapshot_path(), blob)
    }

    pub fn load_snapshot(&self) -> Option<IndexStore> {
        let data = fs::read(self.snapshot_path()).ok()?;
        decode_from_slice::<IndexStore, _>(&data, standard()).ok().map(|v| v.0)
    }

    /* =====================
       JOURNAL
    ===================== */

    pub fn append_journal(&self, entry: JournalEntry) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .append(true)
            .create(true)
            .open(self.journal_path())?;

        let payload = encode_to_vec(entry, standard()).unwrap();
        let len = payload.len() as u32;

        f.write_all(&len.to_le_bytes())?;
        f.write_all(&payload)?;
        Ok(())
    }

    pub fn replay_journal(&self, store: &mut IndexStore) -> std::io::Result<()> {
        let mut f = match File::open(self.journal_path()) {
            Ok(f) => f,
            Err(_) => return Ok(()),
        };

        loop {
            let mut len_buf = [0u8; 4];
            if f.read_exact(&mut len_buf).is_err() {
                break;
            }

            let len = u32::from_le_bytes(len_buf) as usize;
            let mut blob = vec![0u8; len];
            f.read_exact(&mut blob)?;

            if let Ok((entry, _)) = decode_from_slice::<JournalEntry, _>(&blob, standard()) {
                entry.apply(store);
            }
        }

        Ok(())
    }

    pub fn clear_journal(&self) -> std::io::Result<()> {
        fs::remove_file(self.journal_path()).or(Ok(()))
    }
}

/* =====================
   JOURNAL MODEL
===================== */

use crate::index::model::{FileId, FileRecord};

#[derive(Debug, Encode, Decode)]
pub enum JournalEntry {
    Upsert(FileRecord),
    Delete(FileId),
}

impl JournalEntry {
    pub fn apply(self, store: &mut IndexStore) {
        match self {
            JournalEntry::Upsert(rec) => store.upsert(rec),
            JournalEntry::Delete(id) => {
                let path = store.by_id.get(&id).map(|r| r.path.clone());
                if let Some(p) = path {
                    store.remove_by_path(&p);
                }
            }
        }
    }
}
