use proc_macro2::TokenStream;
use quote::quote;
use syn::FnArg;

use super::ToTsType;
use crate::unsupported;

impl ToTsType for FnArg {
    fn to_tstype(&self) -> TokenStream {
        match self {
            FnArg::Typed(arg) => {
                let ident = arg.pat.to_tstype();
                let ty = arg.ty.to_tstype();
                quote! { #ident: #ty }
            },
            _ => unsupported!(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{FnArg, ToTsType, TokenStream};
    use quote::quote;

    #[rstest]
    #[case(quote! {
        num: number
    }, quote! {
        num: u32
    })]
    #[case(quote! {
        str: string
    }, quote! {
        str: String
    })]
    #[case(quote! {
        input: InputType
    }, quote! {
        input: InputType
    })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: FnArg = syn::parse2(item).unwrap();
        assert_eq!(expected.to_string(), item.to_tstype().to_string());
    }
}
