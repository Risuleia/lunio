use std::fs;

use crate::thumbs::persist::{ThumbIndex, ThumbJournal};

#[derive(Clone, Copy)]
pub struct EvictionPolicy {
    pub max_bytes: u64,
    pub max_entries: usize,
}

impl Default for EvictionPolicy {
    fn default() -> Self {
        Self {
            max_bytes: 5 * 1024 * 1024 * 1024, // 5GB default
            max_entries: 200_000,
        }
    }
}

pub fn enforce_limits(
    idx: &mut ThumbIndex,
    policy: &EvictionPolicy,
    persist: &super::persist::ThumbPersistence,
) {

    // ✅ Collect ownership-safe eviction list (NO REFERENCES)
    let mut entries: Vec<(String, u64)> = idx.entries
        .iter()
        .map(|(k, v)| (k.clone(), v.last_accessed))
        .collect();

    // Sort least recently used FIRST
    entries.sort_by_key(|(_, access)| *access);

    // Compute total size
    let mut total_bytes: u64 = idx.entries
        .values()
        .filter_map(|v| fs::metadata(&v.png_path).ok().map(|m| m.len()))
        .sum();

    // Evict while limits exceeded
    for (cache_key, _) in entries {

        let over_limits =
            total_bytes > policy.max_bytes ||
            idx.entries.len() > policy.max_entries;

        if !over_limits {
            break;
        }

        // ✅ REMOVE SAFE (no borrowed refs exist)
        if let Some(meta) = idx.entries.remove(&cache_key) {

            let size = fs::metadata(&meta.png_path)
                .map(|m| m.len())
                .unwrap_or(0);

            let _ = fs::remove_file(&meta.png_path);

            total_bytes = total_bytes.saturating_sub(size);

            persist.append(ThumbJournal::Remove {
                cache_key: meta.cache_key.clone(),
            }).ok();
        }
    }
}