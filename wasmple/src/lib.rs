mod buffer;
mod console;

use buffer::convert;
use buffer::BufferPtr;

#[no_mangle]
pub extern "C" fn revstr(input_ptr: BufferPtr) -> BufferPtr {
    match convert::into::string(input_ptr) {
        None => 0,
        Some(input_str) => {
            let output_str: String = input_str.chars().rev().collect();
            convert::from::string(output_str)
        }
    }
}
