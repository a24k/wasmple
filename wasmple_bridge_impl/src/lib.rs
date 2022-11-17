use proc_macro2::TokenStream;
use syn::Item;

pub fn wasmple_bridge_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    syn::parse2::<Item>(item.clone()).map_or((), |item| {
        analyze(&item);
    });
    item
}

fn analyze(item: &Item) -> String {
    match item {
        Item::Type(item) => format!("type: {:?}", item),
        Item::Struct(item) => format!("struct: {:?}", item),
        Item::Fn(item) => format!("fn: {:?}", item),
        _ => String::default(),
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{Item, TokenStream};
    use quote::quote;

    #[rstest]
    // empty
    #[case(quote!{ })]
    // Rust tokens
    #[case(quote!{
        pub type TestType = usize;
    })]
    #[case(quote!{
        struct TestStruct {
            num: u32,
            str: String,
        }
    })]
    #[case(quote!{
        pub extern "C" fn test_function(input_ptr: BufferPtr) -> BufferPtr {
            input_ptr
        }
    })]
    // TypeScript tokens, these are also valid as TokenStream in these cases
    #[case(quote!{ export type TestType = number; })]
    #[case(quote!{ export type FnConvertParameters = { a: string, b: string }; })]
    #[case(quote!{ export type FnConvertParameters = (ptr: BufferPtr) => BufferPtr; })]
    fn do_not_modify_the_item(#[case] item: TokenStream) {
        assert_eq!(
            item.to_string(),
            super::wasmple_bridge_impl(TokenStream::new(), item).to_string()
        );
    }

    #[rstest]
    #[case("type", syn::parse2(quote!{
        pub type TestType = usize;
    }).unwrap())]
    #[case("struct", syn::parse2(quote!{
        struct TestStruct {
            num: u32,
            str: String,
        }
    }).unwrap())]
    #[case("fn", syn::parse2(quote!{
        pub extern "C" fn test_function(input_ptr: BufferPtr) -> BufferPtr {
            input_ptr
        }
    }).unwrap())]
    fn analyze_item_type(#[case] expected: String, #[case] item: Item) {
        assert_eq!(
            expected,
            super::analyze(&item)
                .split_once(":")
                .unwrap_or_default()
                .0
        );
    }
}
