use super::alloc;
use super::buffer::BufferPtr;

#[no_mangle]
pub extern "C" fn alloc_u16(len: usize) -> BufferPtr {
    alloc::alloc_u16(len)
}

#[no_mangle]
pub extern "C" fn length(ptr: BufferPtr) -> usize {
    alloc::length(ptr)
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: BufferPtr) {
    alloc::dealloc(ptr)
}
