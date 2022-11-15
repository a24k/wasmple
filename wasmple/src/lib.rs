use itertools::Itertools;

use serde::{Deserialize, Serialize};

use wasmple_console::info;

use wasmple_buffer::convert::JsonConvertee;
use wasmple_buffer::BufferPtr;

#[derive(Serialize, Deserialize, JsonConvertee, Debug, PartialEq, Eq)]
struct FnConvertParameters {
    a: String,
    b: String,
}

#[derive(Serialize, Deserialize, JsonConvertee, Debug, PartialEq, Eq)]
struct FnConvertReturns {
    interleaved: String,
    reversed: String,
}

#[no_mangle]
pub extern "C" fn convert(input_ptr: BufferPtr) -> BufferPtr {
    info!("[wasm] convert {}", input_ptr);
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

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{FnConvertParameters, FnConvertReturns};
    use wasmple_buffer::{from, into, BufferManager};

    #[rstest]
    #[case(
        FnConvertReturns {
            interleaved: String::default(),
            reversed: String::default()
        },
        FnConvertParameters {
            a: String::default(),
            b: String::default()
        })]
    #[case(
        FnConvertReturns {
            interleaved: String::from("12345678"),
            reversed: String::from("87654321")
        },
        FnConvertParameters {
            a: String::from("1357"),
            b: String::from("2468")
        })]
    #[case(
        FnConvertReturns {
            interleaved: String::from("パタトクカシーー🚔🚖"),
            reversed: String::from("🚖🚔ーーシカクトタパ")
        },
        FnConvertParameters {
            a: String::from("パトカー🚔"),
            b: String::from("タクシー🚖")
        })]
    fn convert(#[case] estimated: FnConvertReturns, #[case] input: FnConvertParameters) {
        let input_ptr = from(input);

        match input_ptr {
            None => panic!("input_ptr: Option<BufferPtr> will be Some."),
            Some(input_ptr) => {
                let output_ptr = super::_convert(input_ptr);

                match output_ptr {
                    None => panic!("output_ptr: Option<BufferPtr> will be Some."),
                    Some(output_ptr) => {
                        let output = into::<FnConvertReturns>(output_ptr);

                        match output {
                            None => panic!("output: Option<FnConvertReturns> will be Some."),
                            Some(output) => {
                                assert_eq!(estimated, output);
                            }
                        }

                        BufferManager::lock().dealloc(output_ptr);
                    }
                }

                BufferManager::lock().dealloc(input_ptr);
            }
        }
    }
}
