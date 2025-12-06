use std::{collections::HashMap, fs, io::{Read, Write}, path::{Path, PathBuf}, time::{SystemTime, UNIX_EPOCH}};
use bincode::{Decode, Encode, config::standard, decode_from_slice, encode_to_vec};

use crate::thumbs::model::{ThumbSource, ThumbSpec};

#[derive(Debug, Encode, Decode, Clone)]
pub struct ThumbMeta {
    pub source_path: PathBuf,
    pub source_mtime: u64,
    pub source_size: u64,
    pub size: u32,
    pub cache_key: String,
    pub png_path: PathBuf,
    pub created_at: u64,
    pub last_accessed: u64
}

#[derive(Debug, Encode, Decode, Default)]
pub struct ThumbIndex {
    pub version: u16,
    pub entries: HashMap<String, ThumbMeta>, // key = cache_key
}

impl ThumbMeta {
    pub fn key_for(job: &ThumbSpec) -> String {
        use crate::thumbs::cache::ThumbCache;

        let path = match &job.source {
            ThumbSource::Image(p)
            | ThumbSource::Video(p)
            | ThumbSource::Pdf(p)
            | ThumbSource::Unknown(p) => p,
        };

        ThumbCache::hash_path(path, job.size)
    }

    pub fn from_job(job: &ThumbSpec, png_path: std::path::PathBuf) -> Self {
        let key = Self::key_for(job);

        let path = match &job.source {
            ThumbSource::Image(p)
            | ThumbSource::Video(p)
            | ThumbSource::Pdf(p)
            | ThumbSource::Unknown(p) => p.clone(),
        };

        let meta = std::fs::metadata(&path).ok();

        let (size, mtime) = match meta {
            Some(m) => (
                m.len(),
                m.modified()
                    .ok()
                    .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0)
            ),
            None => (0, 0),
        };

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            source_path: path,
            source_mtime: mtime,
            source_size: size,
            size: job.size,
            cache_key: key,
            png_path,
            created_at: now,
            last_accessed: now
        }
    }

    pub fn is_valid(&self) -> bool {
        let meta = std::fs::metadata(&self.source_path).ok();
        let Some(meta) = meta else {
            return false;
        };

        let mtime = meta.modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        mtime == self.source_mtime && meta.len() == self.source_size && self.png_path.exists()
    }

    pub fn touch(&mut self) {
        self.last_accessed = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }
}

#[derive(Debug, Encode, Decode)]
pub enum ThumbJournal {
    Insert(ThumbMeta),
    Remove { cache_key: String },
}

pub struct ThumbPersistence {
    root: PathBuf,
}

impl ThumbPersistence {
    pub fn new(root: impl AsRef<Path>) -> Self {
        let root = root.as_ref().to_path_buf();
        fs::create_dir_all(&root).ok();
        Self { root }
    }

    fn index_path(&self) -> PathBuf { self.root.join("thumbs.index") }
    fn journal_path(&self) -> PathBuf { self.root.join("thumbs.journal") }

    /* ---------- SNAPSHOT ---------- */

    pub fn load_index(&self) -> ThumbIndex {
        let data = fs::read(self.index_path()).ok();
        data.and_then(|b| decode_from_slice::<ThumbIndex, _>(&b, standard()).ok().map(|x| x.0))
            .unwrap_or_else(|| ThumbIndex { version: 1, ..Default::default() })
    }

    pub fn save_index(&self, idx: &ThumbIndex) -> std::io::Result<()> {
        let blob = encode_to_vec(idx, standard()).unwrap();
        fs::write(self.index_path(), blob)
    }

    /* ---------- JOURNAL ---------- */

    pub fn append(&self, entry: ThumbJournal) -> std::io::Result<()> {
        let blob = encode_to_vec(entry, standard()).unwrap();
        let mut f = fs::OpenOptions::new().create(true).append(true).open(self.journal_path())?;
        let len = blob.len() as u32;
        f.write_all(&len.to_le_bytes())?;
        f.write_all(&blob)?;
        Ok(())
    }

    pub fn replay(&self, idx: &mut ThumbIndex) -> std::io::Result<()> {
        let mut f = match fs::File::open(self.journal_path()) {
            Ok(f) => f,
            Err(_) => return Ok(()),
        };

        loop {
            let mut len_buf = [0u8; 4];
            if f.read_exact(&mut len_buf).is_err() { break; }

            let len = u32::from_le_bytes(len_buf) as usize;
            let mut blob = vec![0; len];
            f.read_exact(&mut blob)?;

            if let Ok((entry, _)) = decode_from_slice::<ThumbJournal, _>(&blob, standard()) {
                match entry {
                    ThumbJournal::Insert(meta) => {
                        idx.entries.insert(meta.cache_key.clone(), meta);
                    }
                    ThumbJournal::Remove { cache_key } => {
                        idx.entries.remove(&cache_key);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn clear_journal(&self) -> std::io::Result<()> {
        fs::remove_file(self.journal_path()).or(Ok(()))
    }
}
