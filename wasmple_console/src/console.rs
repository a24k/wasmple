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
        super::import::console_message(level as u8, utf16.as_ptr(), utf16.len());
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
