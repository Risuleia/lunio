use std::sync::Arc;

use crate::thumbs::{
    model::{ThumbResult, ThumbSpec, ThumbStatus, ThumbSource},
    extractor::{self, ExtractorEnv},
    cache::ThumbCache,
};

pub async fn run_worker(
    job: ThumbSpec,
    cache: ThumbCache,
    env: Arc<ExtractorEnv>,
) -> ThumbResult {

    let key = ThumbCache::hash_path(match &job.source {
        ThumbSource::Image(p)
        | ThumbSource::Video(p)
        | ThumbSource::Pdf(p)
        | ThumbSource::Unknown(p) => p,
    }, job.size);

    // Fast-path: already cached
    if cache.exists(&key) {
        return ThumbResult {
            id: job.id,
            status: ThumbStatus::Completed(cache.cache_path(&key)),
        };
    }

    // Perform extraction
    let data = match job.source.clone() {

        ThumbSource::Image(p) => {
            extractor::extract_image(&p, job.size, &env).await
        }

        ThumbSource::Video(p) => {
            extractor::extract_video(&p, job.size, &env).await
        }

        ThumbSource::Pdf(p) => {
            extractor::extract_pdf(&p, job.size, &env).await
        }

        ThumbSource::Unknown(_) => return ThumbResult {
            id: job.id,
            status: ThumbStatus::Failed("unsupported format".to_string()),
        },
    };

    // Cache and return
    match data {
        Ok(bytes) => match cache.write_atomic(&key, &bytes) {
            Ok(path) => ThumbResult {
                id: job.id,
                status: ThumbStatus::Completed(path),
            },
            Err(e) => ThumbResult {
                id: job.id,
                status: ThumbStatus::Failed(e.to_string()),
            },
        },

        Err(err) => ThumbResult {
            id: job.id,
            status: ThumbStatus::Failed(err),
        },
    }
}
