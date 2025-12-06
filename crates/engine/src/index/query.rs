use crate::index::{model::{FileId, FileRecord}, store::IndexStore, tokenize::tokenize};
use std::{collections::HashSet, path::PathBuf};

/* ===============================
   QUERY DEFINITION
================================ */

#[derive(Debug, Clone)]
pub enum Query {

    // Logical operators
    And(Vec<Query>),
    Or(Vec<Query>),
    Not(Box<Query>),

    // Field filters
    Name(String),
    Tokens(Vec<String>),
    Ext(String),
    InDir(PathBuf),
    PathPrefix(String),

    SizeLess(u64),
    SizeGreater(u64),
}

struct Scored<'a> {
    rec: &'a FileRecord,
    score: i64
}

/* ===============================
   QUERY EXECUTION
================================ */

pub fn execute(store: &IndexStore, q: Query) -> Vec<FileRecord> {
    let set = eval(store, q.clone());

    let mut list = Vec::new();

    for id in set {
        if let Some(rec) = store.get_by_id(&id) {
            let score = score_match(store, &rec, &q);
            list.push(Scored { rec, score });
        }
    }

    list.sort_by_key(|s| -s.score);

    list.into_iter()
        .map(|s| s.rec.clone())
        .collect()
}

fn score_match(store: &IndexStore, rec: &FileRecord, q: &Query) -> i64 {
    let mut score = 0;

    apply_score(store, rec, q, &mut score);

    // Depth bias (prefer shallower)
    let depth = rec.path.components().count() as i64;
    score -= depth * 2;

    // Recency bonus
    let now = chrono::Utc::now().timestamp();
    let age = now - rec.modified_unix as i64;

    if age < 3600 { score += 50; }        // last hour
    else if age < 86400 { score += 20; }  // today
    else if age < 604800 { score += 10; } // last week

    // File type preference (example)
    match rec.ext.as_deref() {
        Some("pdf") => score += 10,
        Some("txt") => score += 5,
        _ => {}
    }

    score
}

fn apply_score(store: &IndexStore, rec: &FileRecord, q: &Query, score: &mut i64) {

    match q {

        Query::Name(name) => {
            if rec.name == *name {
                *score += 100; // exact filename
            } else if is_fuzzy_match(name, &rec.name) {
                *score += 40; // fuzzy filename
            }
        }

        Query::Tokens(toks) => {
            let file_tokens = tokenize(&rec.name);

            for user_tok in toks {
                for file_tok in &file_tokens {
                    if user_tok == file_tok {
                        *score += 40; // exact token match
                    } else if is_fuzzy_match(user_tok, file_tok) {
                        *score += 20; // fuzzy token match
                    }
                }
            }
        }

        Query::And(list) | Query::Or(list) => {
            for sub in list {
                apply_score(store, rec, sub, score);
            }
        }

        Query::Not(inner) => {
            apply_score(store, rec, inner, score);
        }

        _ => {}
    }
}

fn eval(store: &IndexStore, q: Query) -> HashSet<FileId> {
    match q {

        Query::And(list) => {
            let mut iter = list.into_iter().map(|q| eval(store, q));
            let first = iter.next().unwrap_or_default();

            iter.fold(first, |acc, s| {
                acc.intersection(&s).cloned().collect()
            })
        }

        Query::Or(list) => {
            list.into_iter()
                .flat_map(|q| eval(store, q))
                .collect()
        }

        Query::Not(inner) => {
            let all: HashSet<FileId> = store.by_id.keys().cloned().collect();
            let neg = eval(store, *inner);
            all.difference(&neg).cloned().collect()
        }

        Query::Name(name) => lookup(store.by_name.get(&name)),

        Query::Ext(ext) => lookup(store.by_ext.get(&ext)),

        Query::InDir(dir) => lookup(store.by_parent.get(&dir)),

        Query::PathPrefix(pref) => {
            store.by_path.iter()
                .filter(|(p, _)| p.to_string_lossy().starts_with(&pref))
                .map(|(_, id)| id.clone())
                .collect()
        }

        Query::SizeLess(n) => {
            store.by_size
                .range(..n)
                .flat_map(|(_, ids)| ids.iter())
                .cloned()
                .collect()
        }

        Query::SizeGreater(n) => {
            store.by_size
                .range(n+1..)
                .flat_map(|(_, ids)| ids.iter())
                .cloned()
                .collect()
        }

        Query::Tokens(toks) => {
            let mut results = HashSet::new();

            for tok in toks {
                // fast path: exact token index
                if let Some(set) = store.by_token.get(&tok) {
                    results.extend(set.iter().cloned());
                    continue;
                }

                // fuzz: scan token keys
                for (key, set) in &store.by_token {
                    if is_fuzzy_match(&tok, key) {
                        results.extend(set.iter().cloned());
                    }
                }
            }

            results
        }
    }
}

fn lookup(opt: Option<&HashSet<FileId>>) -> HashSet<FileId> {
    opt.cloned().unwrap_or_default()
}

fn is_fuzzy_match(a: &str, b: &str) -> bool {
    levenshtein(a, b) <= 2 || b.starts_with(a) || a.starts_with(b)
}

fn levenshtein(a: &str, b: &str) -> usize {
    let mut costs = vec![0; b.len() + 1];
    for j in 0..=b.len() { costs[j] = j; }

    for (i, ca) in a.chars().enumerate() {
        let mut last = i;
        costs[0] = i + 1;

        for (j, cb) in b.chars().enumerate() {
            let new = if ca == cb { last } else { last + 1 };
            last = costs[j + 1];
            costs[j + 1] = std::cmp::min(
                std::cmp::min(costs[j] + 1, costs[j + 1] + 1),
                new
            );
        }
    }

    costs[b.len()]
}


/* ===============================
   INDEXED OPERATIONS
================================ */

fn by_name(store: &IndexStore, name: &str) -> Vec<FileRecord> {
    match store.by_name.get(name) {
        Some(ids) => ids.iter().filter_map(|id| store.by_id.get(id).cloned()).collect(),
        None => vec![],
    }
}

fn by_ext(store: &IndexStore, ext: &str) -> Vec<FileRecord> {
    match store.by_ext.get(ext) {
        Some(ids) => ids.iter().filter_map(|id| store.by_id.get(id).cloned()).collect(),
        None => vec![],
    }
}

fn by_parent(store: &IndexStore, parent: &PathBuf) -> Vec<FileRecord> {
    match store.by_parent.get(parent) {
        Some(ids) => ids.iter().filter_map(|id| store.by_id.get(id).cloned()).collect(),
        None => vec![],
    }
}

fn by_path_prefix(store: &IndexStore, prefix: &str) -> Vec<FileRecord> {
    store.by_path
        .keys()
        .filter(|p| p.to_string_lossy().starts_with(prefix))
        .filter_map(|p| store.get_by_path(p).cloned())
        .collect()
}

fn by_size_lt(store: &IndexStore, limit: u64) -> Vec<FileRecord> {
    store.by_size
        .range(..limit)
        .flat_map(|(_, ids)| ids.iter())
        .filter_map(|id| store.by_id.get(id).cloned())
        .collect()
}

fn by_size_gt(store: &IndexStore, limit: u64) -> Vec<FileRecord> {
    store.by_size
        .range(limit+1..)
        .flat_map(|(_, ids)| ids.iter())
        .filter_map(|id| store.by_id.get(id).cloned())
        .collect()
}
