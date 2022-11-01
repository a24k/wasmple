mod buffer;
mod console;

use buffer::{BufferManager, BufferPtr};

#[no_mangle]
pub extern "C" fn revstr(input_ptr: BufferPtr) -> BufferPtr {
    let mut manager = BufferManager::lock();

    match manager.get(input_ptr) {
        None => 0,
        Some(arc) => {
            let buf = arc.lock().unwrap();
            let slice = buf.slice::<u16>();

            let input_str = String::from_utf16(slice).unwrap();
            let output_str: String = input_str.chars().rev().collect();
            let utf16: Vec<u16> = output_str.encode_utf16().collect();

            match manager.alloc::<u16>(utf16.len()) {
                None => 0,
                Some(arc) => {
                    let mut buf = arc.lock().unwrap();
                    let slice = buf.slice_mut::<u16>();
                    slice.copy_from_slice(&utf16[..]);

                    buf.ptr()
                },
            }
        }
    }
}
