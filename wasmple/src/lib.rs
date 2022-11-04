mod buffer;
mod console;

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use buffer::BufferPtr;

#[derive(Deserialize)]
struct FnConvertParameters {
    a: String,
    b: String,
}

#[derive(Serialize)]
struct FnConvertReturns {
    interleaved: String,
    reversed: String,
}

#[no_mangle]
pub extern "C" fn convert(input_ptr: BufferPtr) -> BufferPtr {
    _convert(input_ptr).unwrap_or(0)
}

fn _convert(input_ptr: BufferPtr) -> Option<BufferPtr> {
    let input_json: Value = buffer::into(input_ptr)?;
    let input: FnConvertParameters = serde_json::from_value(input_json).ok()?;

    let interleaved: String = input.a.chars().interleave(input.b.chars()).collect();

    let reversed: String = interleaved.chars().rev().collect();

    let output = FnConvertReturns {
        interleaved,
        reversed,
    };

    let output_json = serde_json::to_value(output).ok()?;
    buffer::from(output_json)
}
