use proc_macro2::TokenStream;
use syn::Item;

use super::ToTsType;
use crate::unsupported;

impl ToTsType for Item {
    fn to_tstype(&self) -> TokenStream {
        match self {
            Item::Type(item) => item.to_tstype(),
            Item::Struct(item) => item.to_tstype(),
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
        export type TestStruct = { size: TestType, num: number, str: string?, vec: string[] };
    }, quote! {
        struct TestStruct {
            size: TestType,
            num: u32,
            str: Option<String>,
            vec: Vec<String>,
        }
    })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: Item = syn::parse2(item).unwrap();
        assert_eq!(expected.to_string(), item.to_tstype().to_string());
    }
}
