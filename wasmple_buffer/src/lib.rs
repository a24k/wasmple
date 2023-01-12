pub mod buffer;
pub mod convert;
pub mod manager;

mod define;

mod export;

pub use buffer::{Buffer, BufferPtr};
pub use convert::{from, into};
pub use manager::BufferManager;

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_typescript() -> String {
    export::buffer_clear(); // dummy calling
    wasmple_bridge::generate()
}
