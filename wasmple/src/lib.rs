mod buffer;
mod console;

#[no_mangle]
pub extern "C" fn init() -> bool {
    console::init()
}
