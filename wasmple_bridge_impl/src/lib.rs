mod totstype;

use proc_macro2::TokenStream;
use syn::Item;

use totstype::ToTsType;

pub fn wasmple_bridge_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    if let Ok(item) = syn::parse2::<Item>(item.clone()) {
        item.to_tstype();
    }
    item
}

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
    // TypeScript tokens, these are also valid as TokenStream in these cases
    #[case::typescript(quote! { export type TestType = number; })]
    #[case::typescript(quote! { export type TestStruct = { num: number, str: string }; })]
    #[case::typescript(quote! { export type TestFn = (ptr: BufferPtr) => BufferPtr; })]
    fn do_not_modify_input_item(#[case] item: TokenStream) {
        assert_eq!(
            item.to_string(),
            super::wasmple_bridge_impl(TokenStream::new(), item).to_string()
        );
    }
}
