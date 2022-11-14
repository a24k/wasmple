#[macro_export]
macro_rules! log {
    ($e:expr) => { $crate::log($e.to_string()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::log(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! debug {
    ($e:expr) => { $crate::debug($e.to_string()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::debug(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! info {
    ($e:expr) => { $crate::info($e.to_string()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::info(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! warn {
    ($e:expr) => { $crate::warn($e.to_string()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::warn(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! error {
    ($e:expr) => { $crate::error($e.to_string()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::error(format!($fmt, $($arg)*)) };
}
