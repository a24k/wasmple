use super::alloc;
use super::buffer::BufPtr;

#[no_mangle]
pub extern "C" fn alloc(len: usize) -> BufPtr {
    alloc::alloc(len)
}

#[no_mangle]
pub extern "C" fn free(ptr: BufPtr) -> usize {
    alloc::free(ptr)
}
