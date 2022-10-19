mod buffer;
mod console;

use std::slice;

use buffer::alloc;
use buffer::buffer::BufPtr;

#[no_mangle]
pub extern "C" fn init() -> bool {
    console::init()
}

#[no_mangle]
pub extern "C" fn reverse_string(input_ptr: BufPtr) -> BufPtr {
    let slice = unsafe { slice::from_raw_parts(input_ptr as *const u16, alloc::size_of(input_ptr) / 2) };
    let input_str = String::from_utf16(slice).unwrap();

    let output_str: String = input_str.chars().rev().collect();

    let utf16: Vec<u16> = output_str.encode_utf16().collect();

    let output_ptr = alloc::alloc(utf16.len() * 2);
    let slice = unsafe { slice::from_raw_parts_mut(output_ptr as *mut u16, alloc::size_of(output_ptr) / 2) };
    slice.copy_from_slice(&utf16[..]);

    return output_ptr;
}
