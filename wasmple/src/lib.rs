mod buffer;
mod console;

use buffer::BufferPtr;

#[no_mangle]
pub extern "C" fn revstr(input_ptr: BufferPtr) -> BufferPtr {
    buffer::into::<String>(input_ptr).map_or(0, |input_str| {
        let output_str: String = input_str.chars().rev().collect();
        buffer::from(output_str).unwrap_or(0)
    })
}
