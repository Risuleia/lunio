use std::sync::Arc;
use tokio::sync::{mpsc, Mutex, RwLock};

use crate::thumbs::{
    cache::ThumbCache, evict::{EvictionPolicy, enforce_limits}, extractor::ExtractorEnv, model::{ThumbResult, ThumbSpec, ThumbStatus}, persist::{ThumbIndex, ThumbJournal, ThumbMeta, ThumbPersistence}, scheduler::ThumbScheduler, worker::run_worker
};

pub struct ThumbService {
    scheduler: Arc<Mutex<ThumbScheduler>>,
    result_tx: mpsc::Sender<ThumbResult>,

    index: Arc<RwLock<ThumbIndex>>,
    persist: Arc<ThumbPersistence>,
}

impl ThumbService {

    pub fn start(
        workers: usize,
        cache_dir: impl AsRef<std::path::Path>,
        env: ExtractorEnv,
    ) -> (Self, mpsc::Receiver<ThumbResult>) {

        let (tx, rx) = mpsc::channel(512);
        let scheduler = Arc::new(Mutex::new(ThumbScheduler::new()));
        let cache = ThumbCache::new(&cache_dir);

        // ── Load persistence ──────────────────────────────
        let persist = Arc::new(ThumbPersistence::new(&cache_dir));
        let mut idx = persist.load_index();
        persist.replay(&mut idx).ok();

        // Drop entries whose PNG no longer exists
        idx.entries.retain(|_, meta| meta.png_path.exists());

        // Enforce cache limits at boot
        let policy = EvictionPolicy::default();
        enforce_limits(&mut idx, &policy, &persist);

        // Save clean snapshot
        persist.save_index(&idx).ok();
        persist.clear_journal().ok();

        let index = Arc::new(RwLock::new(idx));
        let env = Arc::new(env);

        // ── Workers ───────────────────────────────────────
        for _ in 0..workers {

            let sched = scheduler.clone();
            let result = tx.clone();

            let cache = cache.clone();
            let env = env.clone();
            let index = index.clone();
            let persist = persist.clone();

            tokio::spawn(async move {
                loop {

                    let job = {
                        let mut s = sched.lock().await;
                        s.pop()
                    };

                    let Some(job) = job else {
                        tokio::time::sleep(std::time::Duration::from_millis(25)).await;
                        continue;
                    };

                    let res = run_worker(job.clone(), cache.clone(), env.clone()).await;

                    // ── Persist success ──────────────────────
                    if let ThumbStatus::Completed(ref png) = res.status {

                        let meta = ThumbMeta::from_job(&job, png.clone());

                        {
                            let mut idx = index.write().await;

                            idx.entries.insert(
                                meta.cache_key.clone(),
                                meta.clone(),
                            );

                            // Enforce eviction after insert
                            enforce_limits(&mut idx, &policy, &persist);
                        }

                        persist.append(ThumbJournal::Insert(meta)).ok();
                    }

                    let _ = result.send(res).await;
                }
            });
        }

        (
            Self {
                scheduler,
                result_tx: tx,
                index,
                persist,
            },
            rx,
        )
    }

    /* =====================
       SUBMISSION WITH WARM CACHE
    ====================== */

    pub async fn submit(&self, job: ThumbSpec) {

        let key = ThumbMeta::key_for(&job);

        // ── Cache HIT ───────────────────────────────
        if let Some(meta) = self.index.write().await.entries.get_mut(&key) {

            if meta.is_valid() {

                // Update LRU
                meta.touch();

                let _ = self.result_tx.send(ThumbResult {
                    id: job.id,
                    status: ThumbStatus::Completed(meta.png_path.clone()),
                }).await;

                return;
            }
        }

        // ── Cache MISS ──────────────────────────────
        let mut s = self.scheduler.lock().await;
        s.push(job);
    }
}
