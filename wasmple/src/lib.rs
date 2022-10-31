mod buffer;
mod console;

use buffer::{BufferPtr, BufferManager};

#[no_mangle]
pub extern "C" fn init() -> bool {
    console::init()
}

#[no_mangle]
pub extern "C" fn reverse_string(input_ptr: BufferPtr) -> BufferPtr {
    let mut manager = BufferManager::lock();

    let arc = manager.buffer(input_ptr);
    let buf = arc.lock().unwrap();
    let slice = buf.slice::<u16>();

    let input_str = String::from_utf16(slice).unwrap();
    let output_str: String = input_str.chars().rev().collect();
    let utf16: Vec<u16> = output_str.encode_utf16().collect();

    let output_ptr = manager.alloc::<u16>(utf16.len());

    let arc = manager.buffer(output_ptr);
    let mut buf = arc.lock().unwrap();
    let slice = buf.slice_mut::<u16>();
    slice.copy_from_slice(&utf16[..]);

    output_ptr
}
