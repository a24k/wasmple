mod buffer;
mod console;

use itertools::Itertools;
use serde_json::Value;

use buffer::BufferPtr;

#[no_mangle]
pub extern "C" fn convert(input_ptr: BufferPtr) -> BufferPtr {
    _convert(input_ptr).unwrap_or(0)
}

fn _convert(input_ptr: BufferPtr) -> Option<BufferPtr> {
    let input_json: Value = serde_json::from_str(&buffer::into::<String>(input_ptr)?).ok()?;

    let input_str_a = input_json["a"].clone();
    let input_str_b = input_json["b"].clone();

    match (input_str_a, input_str_b) {
        (Value::String(input_str_a), Value::String(input_str_b)) => {
            let interleaved_str: String = input_str_a
                .chars()
                .interleave(input_str_b.chars())
                .collect();

            let reversed_str: String = interleaved_str.chars().rev().collect();

            let output_json =
                serde_json::json!({"interleaved": interleaved_str, "reversed": reversed_str});

            buffer::from(serde_json::to_string(&output_json).ok()?)
        }
        _ => None,
    }
}
