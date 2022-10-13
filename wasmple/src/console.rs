use std::panic;
use std::sync::Once;

#[link(wasm_import_module = "console")]
extern "C" {
    fn console_message(level: u8, ptr: *const u16, len: usize);
}

pub fn init() -> bool {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        fn panic_hook(info: &panic::PanicInfo) {
            error(info.to_string());
        }
        panic::set_hook(Box::new(panic_hook));
    });
    ONCE.is_completed()
}

enum LogLevel {
    LOG,
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

fn console_message_with_loglevel(level: LogLevel, msg: String) {
    let utf16: Vec<u16> = msg.encode_utf16().collect();
    unsafe {
        console_message(level as u8, utf16.as_ptr(), utf16.len());
    }
}

#[allow(dead_code)]
pub fn log(msg: String) {
    console_message_with_loglevel(LogLevel::LOG, msg);
}

#[allow(dead_code)]
pub fn debug(msg: String) {
    console_message_with_loglevel(LogLevel::DEBUG, msg);
}

#[allow(dead_code)]
pub fn info(msg: String) {
    console_message_with_loglevel(LogLevel::INFO, msg);
}

#[allow(dead_code)]
pub fn warn(msg: String) {
    console_message_with_loglevel(LogLevel::WARN, msg);
}

#[allow(dead_code)]
pub fn error(msg: String) {
    console_message_with_loglevel(LogLevel::ERROR, msg);
}
