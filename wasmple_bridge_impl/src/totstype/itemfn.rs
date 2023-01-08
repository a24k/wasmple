use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{ItemFn, ReturnType};

use super::ToTsType;

impl ToTsType for ItemFn {
    fn to_tstype(&self) -> TokenStream {
        let ident = format_ident!(
            "Fn{}",
            &self.sig.ident.to_string().to_case(Case::UpperCamel)
        );

        let inputs = self.sig.inputs.iter().map(|fnarg| fnarg.to_tstype());
        let inputs = quote! {
            ( #(#inputs),* )
        };

        let output = &self.sig.output;
        match output {
            ReturnType::Default => {
                quote! { export type #ident = #inputs => void; }
            }
            ReturnType::Type(_, ty) => {
                let ty = ty.to_tstype();
                quote! { export type #ident = #inputs => #ty; }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ItemFn, ToTsType, TokenStream};
    use quote::quote;

    #[rstest]
    #[case(quote! {
        export type FnTestFunctionA = (a: InputTypeA, b: InputTypeB) => void;
    }, quote! {
        fn test_function_a(a: InputTypeA, b: InputTypeB) { }
    })]
    #[case(quote! {
        export type FnTestFunctionB = (input: InputType) => OutputType;
    }, quote! {
        pub extern "C" fn test_function_b(input: InputType) -> OutputType { }
    })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: ItemFn = syn::parse2(item).unwrap();
        assert_eq!(expected.to_string(), item.to_tstype().to_string());
    }
}
