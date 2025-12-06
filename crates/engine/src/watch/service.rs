use std::{collections::{HashMap, HashSet}, path::PathBuf, sync::mpsc::{Receiver, Sender, channel}, thread, time::{Duration, Instant}};

use notify::{EventKind, RecommendedWatcher, Watcher};

use crate::watch::{backend::WatchBackend, event::WatchEvent};

pub struct NotifyBackend {
    watcher: RecommendedWatcher
}

impl NotifyBackend {
    pub fn new(event_tx: Sender<WatchEvent>) -> Self {
        let tx = event_tx.clone();

        let watcher = RecommendedWatcher::new(
            move |res: Result<notify::Event, notify::Error>| {
                if let Ok(ev) = res {
                    if let Some(evt) = map_notify_event(ev.kind, ev.paths) {
                        let _ = tx.send(evt);
                    }
                }
            },
            notify::Config::default()
        )
        .expect("Failed to init notify watcher");

        Self { watcher }
    }
}

impl WatchBackend for NotifyBackend {
    fn run(&mut self, _out: Sender<WatchEvent>) {}

    fn watch(&mut self, path: PathBuf) {
        let _ = self.watcher.watch(&path, notify::RecursiveMode::Recursive);
    }

    fn unwatch(&mut self, path: PathBuf) {
        let _ = self.watcher.unwatch(&path);
    }
}

pub struct WatchService {
    watched: HashSet<PathBuf>
}

impl WatchService {
    pub fn start(
        out: tokio::sync::mpsc::Sender<WatchEvent>
    ) -> Self {
        let (raw_tx, raw_rx) = channel::<WatchEvent>();

        let mut backend = NotifyBackend::new(raw_tx.clone());

        thread::spawn(move || {
            backend.run(raw_tx);
            pump_events(raw_rx, out);
        });

        Self { watched: HashSet::new() }
    }

    pub fn watch(&mut self, path: PathBuf) {
        self.watched.insert(path.clone());
    }

    pub fn unwatch(&mut self, path: &PathBuf) {
        self.watched.remove(path);
    }
}

fn pump_events(
    rx: Receiver<WatchEvent>,
    out: tokio::sync::mpsc::Sender<WatchEvent>
) {
    let mut last_seen: HashMap<String, Instant> = HashMap::new();
    let debounce = Duration::from_millis(250);

    while let Ok(evt) = rx.recv() {
        let key = match &evt {
            WatchEvent::Created(p) => p.to_string_lossy(),
            WatchEvent::Modified(p) => p.to_string_lossy(),
            WatchEvent::Deleted(p) => p.to_string_lossy(),
        }.to_string();

        let now = Instant::now();
        let allow = match last_seen.get(&key) {
            Some(last) => now.duration_since(*last) > debounce,
            None => true
        };

        if allow {
            last_seen.insert(key, now);
            let _ = out.blocking_send(evt);
        }
    }
}

fn map_notify_event(kind: EventKind, paths: Vec<PathBuf>) -> Option<WatchEvent> {
    let p0 = paths.get(0)?.clone();

    let evt = match kind {
        EventKind::Create(_) => WatchEvent::Created(p0),
        EventKind::Modify(_) => WatchEvent::Modified(p0),
        EventKind::Remove(_) => WatchEvent::Deleted(p0),
        _ => return None  
    };

    Some(evt)
}