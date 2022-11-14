enum LogLevel {
    Log,
    Debug,
    Info,
    Warn,
    Error,
}

#[cfg(target_family = "wasm")]
fn console_message_with_loglevel(level: LogLevel, msg: String) {
    let utf16: Vec<u16> = msg.encode_utf16().collect();
    unsafe {
        super::import::console_message(level as u8, utf16.as_ptr(), utf16.len());
    }
}

#[cfg(not(target_family = "wasm"))]
fn console_message_with_loglevel(level: LogLevel, msg: String) {
    match level {
        LogLevel::Log => eprintln!("[Log] {}", msg),
        LogLevel::Debug => eprintln!("[Debug] {}", msg),
        LogLevel::Info => eprintln!("[Info] {}", msg),
        LogLevel::Warn => eprintln!("[Warn] {}", msg),
        LogLevel::Error => eprintln!("[Error] {}", msg),
    }
}

#[allow(dead_code)]
pub fn log(msg: String) {
    console_message_with_loglevel(LogLevel::Log, msg);
}

#[allow(dead_code)]
pub fn debug(msg: String) {
    console_message_with_loglevel(LogLevel::Debug, msg);
}

#[allow(dead_code)]
pub fn info(msg: String) {
    console_message_with_loglevel(LogLevel::Info, msg);
}

#[allow(dead_code)]
pub fn warn(msg: String) {
    console_message_with_loglevel(LogLevel::Warn, msg);
}

#[allow(dead_code)]
pub fn error(msg: String) {
    console_message_with_loglevel(LogLevel::Error, msg);
}
