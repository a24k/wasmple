use proc_macro2::TokenStream;
use quote::quote;
use syn::Pat;

use super::ToTsType;
use crate::unsupported;

impl ToTsType for Pat {
    fn to_tstype(&self) -> TokenStream {
        match self {
            Pat::Ident(ident) => {
                let ident = &ident.ident;
                quote! { #ident }
            }
            _ => unsupported!(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{Pat, ToTsType, TokenStream};
    use quote::quote;

    #[rstest]
    #[case(quote! {
        input
    }, quote! {
        input
    })]
    #[should_panic(expected = "unsupported Lit")]
    #[case(quote! {
        0
    }, quote! {
        0
    })]
    #[should_panic(expected = "unsupported Lit")]
    #[case(quote! {
        "text"
    }, quote! {
        "text"
    })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: Pat = syn::parse2(item).unwrap();
        assert_eq!(expected.to_string(), item.to_tstype().to_string());
    }
}
