use std::{path::PathBuf, sync::mpsc::Sender};

use crate::watch::event::WatchEvent;

pub trait WatchBackend: Send + 'static {
    fn watch(&mut self, path: PathBuf);
    fn unwatch(&mut self, path: PathBuf);
    fn run(&mut self, out: Sender<WatchEvent>);
}