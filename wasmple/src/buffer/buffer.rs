use once_cell::sync::Lazy;
use std::alloc::Layout;
use std::collections::HashMap;
use std::sync::Mutex;

pub type BufPtr = usize;

pub(super) static LAYS: Lazy<Mutex<HashMap<BufPtr, Layout>>> = Lazy::new(|| Mutex::new(HashMap::new()));
