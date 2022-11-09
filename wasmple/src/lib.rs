mod buffer;
mod console;

use itertools::Itertools;

use serde::{Deserialize, Serialize};

use buffer::convert::JsonConvertee;
use buffer::BufferPtr;

#[derive(Serialize, Deserialize)]
struct FnConvertParameters {
    a: String,
    b: String,
}

#[derive(Serialize, Deserialize)]
struct FnConvertReturns {
    interleaved: String,
    reversed: String,
}

impl JsonConvertee for FnConvertParameters {}
impl JsonConvertee for FnConvertReturns {}

#[no_mangle]
pub extern "C" fn convert(input_ptr: BufferPtr) -> BufferPtr {
    console_log!("[wasm] convert");
    _convert(input_ptr).unwrap_or(0)
}

fn _convert(input_ptr: BufferPtr) -> Option<BufferPtr> {
    let input: FnConvertParameters = buffer::into(input_ptr)?;

    let interleaved: String = input.a.chars().interleave(input.b.chars()).collect();

    let reversed: String = interleaved.chars().rev().collect();

    let output = FnConvertReturns {
        interleaved,
        reversed,
    };

    buffer::from(output)
}
