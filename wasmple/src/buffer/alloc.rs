use std::alloc::Layout;

use crate::console;

use super::buffer::{BufPtr, LAYS};

pub fn alloc(len: usize) -> BufPtr {
    assert_ne!(len, 0);

    let align = std::mem::align_of::<u8>();
    let layout = Layout::from_size_align(len, align).unwrap();

    let mut lays = LAYS.lock().unwrap();

    let ptr = unsafe { std::alloc::alloc(layout) };

    assert!(!ptr.is_null());

    let ptr = ptr as BufPtr;
    lays.insert(ptr, layout);

    console::log(format!(
        "[wasm] buffer::alloc\tat 0x{:x}\twith {:?}",
        ptr, layout
    ));
    console::debug(format!("[wasm] buffer::LAYS\t{:?}", lays));

    ptr
}

pub fn size_of(ptr: BufPtr) -> usize {
    let lays = LAYS.lock().unwrap();

    match lays.get(&ptr) {
        Some(layout) => layout.size(),
        None => 0,
    }
}

pub fn free(ptr: BufPtr) -> usize {
    let mut lays = LAYS.lock().unwrap();

    match lays.remove(&ptr) {
        Some(layout) => {
            console::log(format!(
                "[wasm] buffer::free\tat 0x{:x}\twith {:?})",
                ptr, layout
            ));
            console::debug(format!("[wasm] buffer::LAYS\t{:?}", lays));
            unsafe {
                std::alloc::dealloc(ptr as *mut u8, layout);
                layout.size()
            }
        }
        None => 0,
    }
}
