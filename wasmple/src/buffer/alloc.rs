use std::alloc;
use std::alloc::Layout;
use std::mem;

use crate::console;

use super::buffer::{BufPtr, LAYS};

pub(super) fn alloc(len: usize) -> BufPtr {
    assert_ne!(len, 0);

    let align = mem::align_of::<u8>();
    let layout = Layout::from_size_align(len, align).unwrap();

    let mut lays = LAYS.lock().unwrap();

    let ptr = unsafe { alloc::alloc(layout) };

    assert!(!ptr.is_null());

    let ptr = ptr as BufPtr;
    lays.insert(ptr, layout);

    console::log(format!(
        "rs: buffer::alloc\tat 0x{:x}\twith {:?}",
        ptr, layout
    ));
    console::debug(format!("rs: buffer::LAYS\t{:?}", lays));

    ptr
}

pub(super) fn free(ptr: BufPtr) -> usize {
    let mut lays = LAYS.lock().unwrap();

    match lays.remove(&ptr) {
        Some(layout) => {
            console::log(format!(
                "rs: buffer::free\tat 0x{:x}\twith {:?})",
                ptr, layout
            ));
            console::debug(format!("rs: buffer::LAYS\t{:?}", lays));
            unsafe {
                alloc::dealloc(ptr as *mut u8, layout);
                layout.size()
            }
        }
        None => 0,
    }
}
