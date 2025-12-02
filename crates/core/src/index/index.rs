use std::collections::HashMap;

use crate::models::{FileId, FileMeta};

#[derive(Default)]
pub struct SimpleIndex {
    pub files: HashMap<FileId, FileMeta>
}

impl SimpleIndex {
    pub fn new() -> Self {
        Self { files: HashMap::new() }
    }

    pub fn insert(&mut self, meta: FileMeta) {
        self.files.insert(meta.id, meta);
    }

    pub fn remove(&mut self, id: FileId) {
        self.files.remove(&id);
    }

    pub fn get(&self, id: FileId) -> Option<&FileMeta> {
        self.files.get(&id)
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }
    
    pub fn apply_change(&mut self, id: FileId, meta: Option<FileMeta>) {
        match meta {
            Some(m) => {
                self.files.insert(id, m);
            }
            None => {
                self.files.remove(&id);
            }
        }
    }
    
    pub fn apply_full_scan(&mut self, new_files: Vec<FileMeta>) {
        self.files.clear();
        for meta in new_files {
            self.files.insert(meta.id, meta);
        }
    }

    pub fn search(&self, query: &str, limit: usize) -> Vec<FileMeta> {
        let q = query.to_lowercase();
        let mut out = Vec::new();

        let mut entries: Vec<_> = self.files.values().collect();
        entries.sort_by_key(|m| m.path.clone());

        for meta in entries {
            let name = match meta.path.file_name() {
                Some(n) => n.to_string_lossy().to_lowercase(),
                None => continue
            };

            if name.contains(&q) {
                out.push(meta.clone());
                if out.len() >= limit {
                    break;
                }
            }
        }

        out
    }
}