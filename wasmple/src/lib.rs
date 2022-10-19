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
pub extern "C" fn revstr(ptr: BufPtr) {
    let slice = unsafe { slice::from_raw_parts(ptr as *const u16, alloc::size_of(ptr) / 2) };
    console::debug(format!("[wasm] slice = {:?}", slice));

    let str = String::from_utf16(slice).unwrap();
    let revstr: String = str.chars().rev().collect();

    console::log(str);
    console::log(revstr);
}
