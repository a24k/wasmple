use once_cell::sync::Lazy;
use std::alloc;
use std::alloc::Layout;
use std::collections::HashMap;
use std::mem;
use std::sync::Mutex;

use super::console;

static BUFS: Lazy<Mutex<HashMap<BufPtr, BufLen>>> = Lazy::new(|| Mutex::new(HashMap::new()));

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BufPtr(*mut u8);
unsafe impl Send for BufPtr {}
unsafe impl Sync for BufPtr {}

type BufLen = usize;

fn _alloc(size: BufLen) -> *mut u8 {
    assert_ne!(size, 0);

    let align = mem::align_of::<u8>();
    let layout = Layout::from_size_align(size, align).unwrap();
    let ptr = unsafe { alloc::alloc(layout) };

    assert!(!ptr.is_null());

    let ptr = BufPtr(ptr);
    let len = layout.size();

    console::log(format!("rs: alloc(ptr: {:?}, len: 0x{:x}, layout: {:?})", ptr, len, layout));

    BUFS.lock().unwrap().insert(ptr, len);

    ptr.0
}

fn _free(ptr: *mut u8) {
    let len = BUFS.lock().unwrap().remove(&BufPtr(ptr)).unwrap();

    console::log(format!("rs: free(ptr: {:?}, len: 0x{:x})", ptr, len));

    unsafe {
        let align = mem::align_of::<u8>();
        let layout = Layout::from_size_align_unchecked(len, align);
        alloc::dealloc(ptr, layout);
    }
}

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut u8 {
    _alloc(size)
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    _free(ptr);
}
