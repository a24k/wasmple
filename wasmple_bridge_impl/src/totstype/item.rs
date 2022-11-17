use proc_macro2::TokenStream;
use quote::quote;
use syn::Item;

use super::ToTsType;

impl ToTsType for Item {
    fn to_tstype_token_stream(&self) -> TokenStream {
        match self {
            Item::Type(item) => item.to_tstype_token_stream(),
            _ => quote! {},
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
    fn convert_item_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: Item = syn::parse2(item).unwrap();
        assert_eq!(
            expected.to_string(),
            item.to_tstype_token_stream().to_string()
        );
    }
}
