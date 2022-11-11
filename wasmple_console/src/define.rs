#[macro_export]
macro_rules! log {
    ($fmt:expr) => { $crate::log($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::log(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! debug {
    ($fmt:expr) => { $crate::debug($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::debug(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! info {
    ($fmt:expr) => { $crate::info($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::info(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! warn {
    ($fmt:expr) => { $crate::warn($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::warn(format!($fmt, $($arg)*)) };
}

#[macro_export]
macro_rules! error {
    ($fmt:expr) => { $crate::error($fmt.into()) };
    ($fmt:expr, $($arg:tt)*) => { $crate::error(format!($fmt, $($arg)*)) };
}
