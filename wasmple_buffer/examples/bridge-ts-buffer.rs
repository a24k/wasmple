#[cfg(not(target_arch = "wasm32"))]
fn main() {
    print!("{}", wasmple_buffer::generate_typescript());
}
