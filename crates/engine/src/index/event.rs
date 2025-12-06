use crate::index::model::FileId;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum IndexEvent {

    /* ======================
       LIFECYCLE
    ====================== */

    Started {
        root: String,
    },

    Completed,

    Paused,

    Resumed,

    /* ======================
       PROGRESS
    ====================== */

    Progress {
        scanned: u64,
        total: Option<u64>,
    },

    /* ======================
       MUTATIONS
    ====================== */

    Inserted {
        id: FileId,
        path: PathBuf,
    },

    Updated {
        id: FileId,
        path: PathBuf,
    },

    Deleted {
        id: FileId,
        path: PathBuf,
    },

    Moved {
        id: FileId,
        from: PathBuf,
        to: PathBuf,
    },

    /* ======================
       HEALTH
    ====================== */

    Error {
        path: Option<PathBuf>,
        message: String,
    },

    Warning {
        message: String,
    },

    /* ======================
       RECOVERY / SYNC
    ====================== */

    ResyncRequested,

    RebuildTriggered,

    SnapshotLoaded {
        entries: u64,
    },

    JournalReplayed {
        entries: u64,
    },
}
