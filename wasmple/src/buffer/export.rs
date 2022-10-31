use super::BufferManager;
use super::BufferPtr;

#[no_mangle]
pub extern "C" fn alloc_u16(len: usize) -> BufferPtr {
    match BufferManager::lock().alloc::<u16>(len) {
        None => 0,
        Some(arc) => arc.lock().unwrap().ptr(),
    }
}

#[no_mangle]
pub extern "C" fn length_u16(ptr: BufferPtr) -> usize {
    BufferManager::lock().length::<u16>(ptr)
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: BufferPtr) {
    BufferManager::lock().dealloc(ptr);
}

#[no_mangle]
pub extern "C" fn clear() {
    BufferManager::lock().clear();
}
