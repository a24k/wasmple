use itertools::Itertools;

use serde::{Deserialize, Serialize};

use wasmple_console::info;

use wasmple_bridge::wasmple_bridge;

use wasmple_buffer::convert::JsonConvertee;
use wasmple_buffer::BufferPtr;

#[wasmple_bridge]
#[derive(Serialize, Deserialize, JsonConvertee, Debug, PartialEq, Eq)]
struct FnConvertParameters {
    a: String,
    b: String,
}

#[wasmple_bridge]
#[derive(Serialize, Deserialize, JsonConvertee, Debug, PartialEq, Eq)]
struct FnConvertReturns {
    interleaved: String,
    reversed: String,
}

#[wasmple_bridge]
#[no_mangle]
pub extern "C" fn convert(input_ptr: BufferPtr) -> BufferPtr {
    info!("[wasm] convert {}", input_ptr);
    _convert(input_ptr).unwrap_or(0)
}

fn _convert(input_ptr: BufferPtr) -> Option<BufferPtr> {
    let input: FnConvertParameters = wasmple_buffer::into(input_ptr)?;

    let interleaved = _interleave(input.a, input.b);
    let reversed = _reverse(interleaved.clone());

    let output = FnConvertReturns {
        interleaved,
        reversed,
    };

    wasmple_buffer::from(output)
}

fn _interleave(input_a: String, input_b: String) -> String {
    input_a.chars().interleave(input_b.chars()).collect()
}

fn _reverse(input: String) -> String {
    input.chars().rev().collect()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn generate_typescript() -> String {
    wasmple_bridge::generate()
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{FnConvertParameters, FnConvertReturns};
    use wasmple_buffer::{from, into, BufferManager};

    #[rstest]
    #[case("", "", "")]
    #[case("abc", "abc", "")]
    #[case("123", "", "123")]
    #[case("12345678", "1357", "2468")]
    #[case("a1b2c345", "abc", "12345")]
    #[case("a1b2c3de", "abcde", "123")]
    #[case("パタトクカシーー🚔🚖", "パトカー🚔", "タクシー🚖")]
    fn interleave(#[case] estimated: String, #[case] input_a: String, #[case] input_b: String) {
        assert_eq!(estimated, super::_interleave(input_a, input_b));
    }

    #[rstest]
    #[case("", "")]
    #[case("87654321", "12345678")]
    #[case("おえういあ", "あいうえお")]
    #[case("土金木水火月日", "日月火水木金土")]
    #[case("😋😊😁", "😁😊😋")]
    fn reverse(#[case] estimated: String, #[case] input: String) {
        assert_eq!(estimated, super::_reverse(input));
    }

    #[rstest]
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
