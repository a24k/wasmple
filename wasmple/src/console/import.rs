#[link(wasm_import_module = "console")]
extern "C" {
    pub(super) fn console_message(level: u8, ptr: *const u16, len: usize);
}
