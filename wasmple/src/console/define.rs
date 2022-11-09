#[macro_export]
macro_rules! console_log {
    ($fmt:expr) => { $crate::console::log($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::console::log(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! console_debug {
    ($fmt:expr) => { $crate::console::debug($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::console::debug(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! console_info {
    ($fmt:expr) => { $crate::console::info($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::console::info(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! console_warn {
    ($fmt:expr) => { $crate::console::error($fmt.warn()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::console::warn(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! console_error {
    ($fmt:expr) => { $crate::console::error($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::console::error(format!($fmt, $($arg)*)) };
}
