use proc_macro2::TokenStream;
use syn::Item;

use crate::unsupported;
use super::ToTsType;

impl ToTsType for Item {
    fn to_tstype_token_stream(&self) -> TokenStream {
        match self {
            Item::Type(item) => item.to_tstype_token_stream(),
            Item::Struct(item) => item.to_tstype_token_stream(),
            _ => unsupported!(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{Item, ToTsType, TokenStream};
    use quote::quote;

    #[rstest]
    #[case(quote! {
        export type TestType = number;
    }, quote! {
        pub type TestType = usize;
    })]
    #[case(quote! {
        export type TestStruct = { num: number, str: string };
    }, quote! {
        struct TestStruct {
            num: u32,
            str: String,
        }
    })]
    fn convert_item_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: Item = syn::parse2(item).unwrap();
        assert_eq!(
            expected.to_string(),
            item.to_tstype_token_stream().to_string()
        );
    }
}
