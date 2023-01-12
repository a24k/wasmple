use std::panic;
use std::sync::Once;

use wasmple_bridge::wasmple_bridge;

#[wasmple_bridge]
#[no_mangle]
pub extern "C" fn console_set_panic_hook() -> bool {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        fn panic_hook(info: &panic::PanicInfo) {
           super::error(info.to_string());
        }
        panic::set_hook(Box::new(panic_hook));
    });
    ONCE.is_completed()
}
