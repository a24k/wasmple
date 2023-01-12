use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, Variant};

use super::ToTsType;
use crate::unsupported;

impl ToTsType for Variant {
    fn to_tstype(&self) -> TokenStream {
        let ident = &self.ident;

        match &self.fields {
            Fields::Unit => (),
            _ => unsupported!(self),
        }

        match &self.discriminant {
            None => (),
            _ => unsupported!(self),
        }

        quote! { #ident }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ToTsType, TokenStream, Variant};
    use quote::quote;

    #[rstest]
    #[case(quote! { ONE }, quote! { ONE })]
    #[case(quote! { TWO }, quote! { TWO })]
    #[should_panic(expected = "unsupported Variant")]
    #[case(quote! { }, quote! { TWO = 2 })]
    #[should_panic(expected = "unsupported Variant")]
    #[case(quote! { }, quote! { TWO(SomeType) })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: Variant = syn::parse2(item).unwrap();
        assert_eq!(expected.to_string(), item.to_tstype().to_string());
    }
}
