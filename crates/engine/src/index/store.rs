use std::{
    collections::{HashMap, HashSet, BTreeMap},
    path::PathBuf,
};

use bincode::{Decode, Encode};

use crate::index::{model::{FileId, FileRecord}, tokenize::tokenize};


#[derive(Encode, Decode)]
pub struct IndexStore {

    /* ===========
       CORE TABLE
       =========== */

    pub by_id: HashMap<FileId, FileRecord>,
    pub by_path: HashMap<PathBuf, FileId>,

    /* ===========
       INDICES
       =========== */

    pub by_name: HashMap<String, HashSet<FileId>>,
    pub by_ext: HashMap<String, HashSet<FileId>>,
    pub by_parent: HashMap<PathBuf, HashSet<FileId>>,
    pub by_type: HashMap<bool, HashSet<FileId>>, // true = dir, false = file

    // Range queries
    pub by_size: BTreeMap<u64, HashSet<FileId>>,
    pub by_mtime: BTreeMap<u64, HashSet<FileId>>, // unix timestamp

    pub by_token: HashMap<String, HashSet<FileId>>,

    // Tombstones for fast deletes
    pub deleted: HashSet<FileId>,
}

impl IndexStore {
    pub fn new() -> Self {
        Self {
            by_id: HashMap::new(),
            by_path: HashMap::new(),

            by_name: HashMap::new(),
            by_ext: HashMap::new(),
            by_parent: HashMap::new(),
            by_type: HashMap::new(),

            by_size: BTreeMap::new(),
            by_mtime: BTreeMap::new(),

            by_token: HashMap::new(),

            deleted: HashSet::new(),
        }
    }

    /* ===============
       INSERT / UPDATE
       =============== */

    pub fn upsert(&mut self, rec: FileRecord) {

        // Remove old record
        if let Some(old) = self.by_path.get(&rec.path).and_then(|id| self.by_id.remove(id)) {
            self.remove_from_indices(&old);
        }

        let id = rec.id.clone();

        // Core tables
        self.by_path.insert(rec.path.clone(), id.clone());
        self.by_id.insert(id.clone(), rec.clone());

        // Index by name
        self.by_name
            .entry(rec.name.clone())
            .or_default()
            .insert(id.clone());

        // Index by extension
        if let Some(ext) = &rec.ext {
            self.by_ext.entry(ext.clone()).or_default().insert(id.clone());
        }

        // Index by parent directory
        if let Some(parent) = rec.path.parent() {
            self.by_parent
                .entry(parent.to_path_buf())
                .or_default()
                .insert(id.clone());
        }

        // Index by type
        self.by_type
            .entry(rec.is_dir)
            .or_default()
            .insert(id.clone());

        // Index by size
        self.by_size
            .entry(rec.size)
            .or_default()
            .insert(id.clone());

        // Index by mtime
        self.by_mtime
            .entry(rec.modified_unix)
            .or_default()
            .insert(id.clone());

        
        for tok in tokenize(&rec.name) {
            self.by_token
                .entry(tok)
                .or_default()
                .insert(id.clone());
        }

        // Clear tombstone if existed
        self.deleted.remove(&id);
    }

    /* =========
       DELETION
       ========= */

    pub fn remove_by_path(&mut self, path: &PathBuf) -> Option<FileRecord> {
        let id = self.by_path.remove(path)?;
        let rec = self.by_id.remove(&id)?;
        self.remove_from_indices(&rec);
        self.deleted.insert(id.clone());
        Some(rec)
    }

    fn remove_from_indices(&mut self, rec: &FileRecord) {

        let id = &rec.id;

        // Name
        if let Some(set) = self.by_name.get_mut(&rec.name) {
            set.remove(id);
        }

        // Extension
        if let Some(ext) = &rec.ext {
            if let Some(set) = self.by_ext.get_mut(ext) {
                set.remove(id);
            }
        }

        // Parent
        if let Some(parent) = rec.path.parent() {
            if let Some(set) = self.by_parent.get_mut(parent) {
                set.remove(id);
            }
        }

        // Type
        if let Some(set) = self.by_type.get_mut(&rec.is_dir) {
            set.remove(id);
        }

        // Size
        if let Some(set) = self.by_size.get_mut(&rec.size) {
            set.remove(id);
        }

        // Time
        if let Some(set) = self.by_mtime.get_mut(&rec.modified_unix) {
            set.remove(id);
        }

        
        for tok in tokenize(&rec.name) {
            if let Some(set) = self.by_token.get_mut(&tok) {
                set.remove(id);
            }
        }
    }

    /* ============
       DIRECT ACCESS
       ============ */

    pub fn get_by_path(&self, path: &PathBuf) -> Option<&FileRecord> {
        self.by_path.get(path).and_then(|id| self.by_id.get(id))
    }

    pub fn get_by_id(&self, id: &FileId) -> Option<&FileRecord> {
        self.by_id.get(id)
    }

    pub fn all(&self) -> impl Iterator<Item=&FileRecord> {
        self.by_id.values()
    }
}
