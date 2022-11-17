use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemType, Type};

use super::ToTsType;

impl ToTsType for ItemType {
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

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ItemType, ToTsType, TokenStream};
    use quote::quote;

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
}
