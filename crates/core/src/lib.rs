pub mod models;
pub mod fs;
pub mod index;
pub mod engine;
pub mod thumbnails;

pub use engine::runtime::EngineRuntime;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// #[cfg(test)]
// mod tests;
