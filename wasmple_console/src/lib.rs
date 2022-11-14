mod console;

mod define;

#[cfg(target_family = "wasm")]
mod export;

#[cfg(target_family = "wasm")]
mod import;

pub use console::{debug, error, info, log, warn};
