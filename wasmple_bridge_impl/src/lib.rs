mod totstype;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

use totstype::ToTsType;

pub use inventory;

pub fn wasmple_bridge_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let Ok(parsed) = syn::parse2::<Item>(item.clone()) else { unsupported!(item) };

    let script = parsed.to_tstype().to_string();

    quote! {
        #item

        wasmple_bridge_impl::inventory::submit!(wasmple_bridge_impl::TsString::new(#script));
    }
}

pub struct TsString {
    #[allow(dead_code)] // temporary avoid warning
    script: &'static str,
}

impl TsString {
    pub const fn new(script: &'static str) -> Self {
        TsString { script }
    }
}

inventory::collect!(TsString);

#[macro_export]
macro_rules! unsupported {
    ($e:expr) => {
        panic!("[wasmple_bridge] unsupported {:?}", $e)
    };
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::TokenStream;
    use quote::quote;

    #[rstest]
    // empty
    #[should_panic(expected = "unsupported TokenStream")]
    #[case::empty(quote! {})]
    // Rust tokens
    #[case::rust(quote! {
        pub type TestType = usize;
    })]
    #[case::rust(quote! {
        struct TestStruct {
            num: u32,
            str: String,
        }
    })]
    #[should_panic(expected = "unsupported Fn")]
    #[case::rust(quote! {
        pub extern "C" fn test_function(input_ptr: BufferPtr) -> BufferPtr {
            input_ptr
        }
    })]
    fn starts_with_input_item(#[case] item: TokenStream) {
        let input = item.to_string();
        let output = super::wasmple_bridge_impl(TokenStream::new(), item).to_string();
        assert!(output.starts_with(&input));
    }
}
