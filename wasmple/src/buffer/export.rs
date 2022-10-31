use super::BufferPtr;
use super::BufferManager;

#[no_mangle]
pub extern "C" fn alloc_u16(len: usize) -> BufferPtr {
    BufferManager::lock().alloc::<u16>(len)
}

#[no_mangle]
pub extern "C" fn length_u16(ptr: BufferPtr) -> usize {
    BufferManager::lock().length::<u16>(ptr)
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: BufferPtr) {
    BufferManager::lock().dealloc(ptr)
}
