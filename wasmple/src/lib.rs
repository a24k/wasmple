use itertools::Itertools;

use serde::{Deserialize, Serialize};

use wasmple_console::info;

use wasmple_buffer::convert::JsonConvertee;
use wasmple_buffer::BufferPtr;

#[derive(Serialize, Deserialize, JsonConvertee)]
struct FnConvertParameters {
    a: String,
    b: String,
}

#[derive(Serialize, Deserialize, JsonConvertee)]
struct FnConvertReturns {
    interleaved: String,
    reversed: String,
}

#[no_mangle]
pub extern "C" fn convert(input_ptr: BufferPtr) -> BufferPtr {
    info!("[wasm] convert");
    _convert(input_ptr).unwrap_or(0)
}

fn _convert(input_ptr: BufferPtr) -> Option<BufferPtr> {
    let input: FnConvertParameters = wasmple_buffer::into(input_ptr)?;

    let interleaved: String = input.a.chars().interleave(input.b.chars()).collect();

    let reversed: String = interleaved.chars().rev().collect();

    let output = FnConvertReturns {
        interleaved,
        reversed,
    };

    wasmple_buffer::from(output)
}
