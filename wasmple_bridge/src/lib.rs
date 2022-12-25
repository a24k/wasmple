pub use wasmple_bridge_attribute::wasmple_bridge;

pub use inventory::submit;

pub struct TsScript {
    #[allow(dead_code)] // temporary avoid warning
    script: &'static str,
}

impl TsScript {
    pub const fn new(script: &'static str) -> Self {
        TsScript { script }
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate() -> String {
    inventory::iter::<TsScript>
        .into_iter()
        .fold(String::new(), |mut str, ts| {
            str += ts.script;
            str += "\n";
            str
        })
}

#[cfg(not(target_arch = "wasm32"))]
inventory::collect!(TsScript);
