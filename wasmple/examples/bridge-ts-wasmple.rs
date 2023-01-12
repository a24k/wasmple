#[cfg(not(target_arch = "wasm32"))]
fn main() {
    print!("{}", wasmple::generate_typescript());
}
