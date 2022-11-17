use proc_macro2::TokenStream;
use quote::quote;
use syn::{Item, ItemType, Type, TypePath};

pub fn wasmple_bridge_impl(_attr: TokenStream, item: TokenStream) -> TokenStream {
    syn::parse2::<Item>(item.clone()).map_or((), |item| {
        analyze(&item);
    });
    item
}

fn analyze(item: &Item) -> TokenStream {
    match item {
        Item::Type(item) => item.to_tstype_token_stream(),
        _ => TokenStream::new(),
    }
}

trait ToTypeScriptType {
    fn to_tstype_token_stream(&self) -> TokenStream;
}

impl ToTypeScriptType for ItemType {
    fn to_tstype_token_stream(&self) -> TokenStream {
        let ident = &self.ident;

        let type_token = match self.ty.as_ref() {
            Type::Path(path) => path.to_tstype_token_stream(),
            _ => quote! {},
        };
        if type_token.is_empty() {
            return quote! {};
        }

        quote! { export type #ident = #type_token ; }
    }
}

impl ToTypeScriptType for TypePath {
    fn to_tstype_token_stream(&self) -> TokenStream {
        let segs = &self.path.segments;

        if segs.len() != 1 {
            return quote! {};
        }

        segs.first().map_or(quote! {}, |segment| {
            match segment.ident.to_string().as_str() {
                "usize" => quote! { number },
                "i8" => quote! { number },
                "u8" => quote! { number },
                "i16" => quote! { number },
                "u16" => quote! { number },
                "i32" => quote! { number },
                "u32" => quote! { number },
                "i64" => quote! { number },
                "u64" => quote! { number },
                "f32" => quote! { number },
                "f64" => quote! { number },
                "bool" => quote! { boolean },
                "String" => quote! { string },
                _ => quote! {},
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ItemType, ToTypeScriptType, TokenStream, TypePath};
    use quote::quote;

    #[rstest]
    // empty
    #[case(quote! {})]
    // Rust tokens
    #[case(quote! {
        pub type TestType = usize;
    })]
    #[case(quote! {
        struct TestStruct {
            num: u32,
            str: String,
        }
    })]
    #[case(quote! {
        pub extern "C" fn test_function(input_ptr: BufferPtr) -> BufferPtr {
            input_ptr
        }
    })]
    // TypeScript tokens, these are also valid as TokenStream in these cases
    #[case(quote! { export type TestType = number; })]
    #[case(quote! { export type FnConvertParameters = { a: string, b: string }; })]
    #[case(quote! { export type FnConvertParameters = (ptr: BufferPtr) => BufferPtr; })]
    fn do_not_modify_the_item(#[case] item: TokenStream) {
        assert_eq!(
            item.to_string(),
            super::wasmple_bridge_impl(TokenStream::new(), item).to_string()
        );
    }

    #[rstest]
    #[case(quote! {}, quote! {
        pub type TestType = unknown;
    })]
    #[case(quote! {
        export type TestType = number;
    }, quote! {
        pub type TestType = usize;
    })]
    #[case(quote! {
        export type TestType = boolean;
    }, quote! {
        pub type TestType = bool;
    })]
    #[case(quote! {
        export type TestType = string;
    }, quote! {
        pub type TestType = String;
    })]
    fn convert_type_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: ItemType = syn::parse2(item).unwrap();
        assert_eq!(
            expected.to_string(),
            item.to_tstype_token_stream().to_string()
        );
    }

    #[rstest]
    #[case(quote! {}, quote! { unknown })]
    #[case(quote! {}, quote! { unknown::unknown })]
    #[case(quote! {}, quote! { usize::usize })]
    #[case(quote! { number }, quote! { usize })]
    #[case(quote! { number }, quote! { i8 })]
    #[case(quote! { number }, quote! { u8 })]
    #[case(quote! { number }, quote! { i16 })]
    #[case(quote! { number }, quote! { u16 })]
    #[case(quote! { number }, quote! { i32 })]
    #[case(quote! { number }, quote! { u32 })]
    #[case(quote! { number }, quote! { i64 })]
    #[case(quote! { number }, quote! { u64 })]
    #[case(quote! { number }, quote! { f32 })]
    #[case(quote! { number }, quote! { f64 })]
    #[case(quote! { boolean }, quote! { bool })]
    #[case(quote! { string }, quote! { String })]
    fn convert_typepath_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: TypePath = syn::parse2(item).unwrap();
        assert_eq!(
            expected.to_string(),
            item.to_tstype_token_stream().to_string()
        );
    }
}
