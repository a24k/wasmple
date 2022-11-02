use super::{BufferManager, BufferPtr};

pub(super) fn from(ptr: BufferPtr) -> Option<String> {
    match BufferManager::lock().get(ptr) {
        None => None,
        Some(arc) => {
            let buf = arc.lock().unwrap();
            let slice = buf.slice::<u16>();
            String::from_utf16(slice).ok()
        }
    }
}

pub(super) fn into(str: String) -> BufferPtr {
    let utf16: Vec<u16> = str.encode_utf16().collect();

    match BufferManager::lock().alloc::<u16>(utf16.len()) {
        None => 0,
        Some(arc) => {
            let mut buf = arc.lock().unwrap();
            let slice = buf.slice_mut::<u16>();
            slice.copy_from_slice(&utf16[..]);

            buf.ptr()
        }
    }
}
