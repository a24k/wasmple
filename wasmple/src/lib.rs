mod buffer;
mod console;

use buffer::BufferPtr;
use itertools::Itertools;

#[no_mangle]
pub extern "C" fn interleave(input_ptr_a: BufferPtr, input_ptr_b: BufferPtr) -> BufferPtr {
    let input_str_a = buffer::into::<String>(input_ptr_a).unwrap_or(String::default());
    let input_str_b = buffer::into::<String>(input_ptr_b).unwrap_or(String::default());

    let output_str: String = input_str_a
        .chars()
        .interleave(input_str_b.chars())
        .collect();

    buffer::from(output_str).unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn reverse(input_ptr: BufferPtr) -> BufferPtr {
    buffer::into::<String>(input_ptr).map_or(0, |input_str| {
        let output_str: String = input_str.chars().rev().collect();
        buffer::from(output_str).unwrap_or(0)
    })
}
