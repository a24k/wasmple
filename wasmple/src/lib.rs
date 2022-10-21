mod buffer;
mod console;

use buffer::alloc;
use buffer::alloc::SliceTypes;
use buffer::buffer::BufferPtr;

#[no_mangle]
pub extern "C" fn init() -> bool {
    console::init()
}

#[no_mangle]
pub extern "C" fn reverse_string(input_ptr: BufferPtr) -> BufferPtr {
    let utf16: Vec<u16>;

    {
        let buffers = buffer::alloc::BUFFERS.lock().unwrap();
        if let SliceTypes::U16(slice) = buffers.get(&input_ptr).unwrap().slice() {
            let input_str = String::from_utf16(slice).unwrap();
            let output_str: String = input_str.chars().rev().collect();

            utf16 = output_str.encode_utf16().collect();
        } else {
            return 0;
        }
    }

    let output_ptr = alloc::alloc_u16(utf16.len());

    {
        let mut buffers = buffer::alloc::BUFFERS.lock().unwrap();
        if let SliceTypes::U16M(slice) = buffers.get_mut(&output_ptr).unwrap().slice_mut() {
            slice.copy_from_slice(&utf16[..]);
            output_ptr
        } else {
            0
        }
    }
}
