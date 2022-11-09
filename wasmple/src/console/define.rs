#[macro_export]
#[allow(unused_macros)]
macro_rules! console_log {
    ($str:expr) => { $crate::console::log($str) };
    ($($arg:tt)*) => { $crate::console::log(format!($($arg)*)) };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_debug {
    ($str:expr) => { $crate::console::debug($str) };
    ($($arg:tt)*) => { $crate::console::debug(format!($($arg)*)) };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_info {
    ($str:expr) => { $crate::console::info($str) };
    ($($arg:tt)*) => { $crate::console::info(format!($($arg)*)) };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_warn {
    ($str:expr) => { $crate::console::warn($str) };
    ($($arg:tt)*) => { $crate::console::warn(format!($($arg)*)) };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! console_error {
    ($str:expr) => { $crate::console::error($str) };
    ($($arg:tt)*) => { $crate::console::error(format!($($arg)*)) };
}
