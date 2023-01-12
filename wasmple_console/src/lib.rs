mod console;

mod define;

mod export;

#[cfg(target_family = "wasm")]
mod import;

pub use console::{debug, error, info, log, warn};

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_typescript() -> String {
    export::console_set_panic_hook(); // dummy calling
    wasmple_bridge::generate()
}
