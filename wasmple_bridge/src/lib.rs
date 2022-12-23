pub use wasmple_bridge_attribute::wasmple_bridge;

pub use inventory;

pub struct TsString {
    #[allow(dead_code)] // temporary avoid warning
    script: &'static str,
}

impl TsString {
    pub const fn new(script: &'static str) -> Self {
        TsString { script }
    }
}

#[cfg(not(target_arch = "wasm32"))]
inventory::collect!(TsString);
