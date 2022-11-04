mod buffer;
mod console;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use buffer::{BufferConverter, BufferPtr};

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

impl BufferConverter<FnConvertParameters> for FnConvertParameters {
    fn from(ptr: BufferPtr) -> Option<FnConvertParameters> {
        serde_json::from_str(&buffer::into::<String>(ptr)?).ok()
    }

    fn into(&self) -> Option<BufferPtr> {
        None
    }
}

impl BufferConverter<FnConvertReturns> for FnConvertReturns {
    fn from(_ptr: BufferPtr) -> Option<FnConvertReturns> {
        None
    }

    fn into(&self) -> Option<BufferPtr> {
        buffer::from(serde_json::to_string(self).ok()?)
    }
}

#[no_mangle]
pub extern "C" fn convert(input_ptr: BufferPtr) -> BufferPtr {
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
