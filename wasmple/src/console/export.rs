use std::panic;
use std::sync::Once;

use crate::console_error;

#[no_mangle]
pub extern "C" fn console_set_panic_hook() -> bool {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        fn panic_hook(info: &panic::PanicInfo) {
            console_error!(info.to_string());
        }
        panic::set_hook(Box::new(panic_hook));
    });
    ONCE.is_completed()
}
