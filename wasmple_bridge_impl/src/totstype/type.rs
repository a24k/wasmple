use proc_macro2::TokenStream;
use syn::Type;

use super::ToTsType;
use crate::unsupported;

impl ToTsType for Type {
    fn to_tstype(&self) -> TokenStream {
        match self {
            Type::Path(path) => path.to_tstype(),
            Type::Paren(paren) => paren.elem.to_tstype(),
            _ => unsupported!(self),
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ToTsType, TokenStream, Type};
    use quote::quote;

    #[rstest]
    #[case(quote! { number }, quote! { usize })]
    #[case(quote! { boolean }, quote! { bool })]
    #[case(quote! { string }, quote! { String })]
    #[case(quote! { number }, quote! { ( usize ) })]
    #[should_panic(expected = "unsupported Slice")]
    #[case(quote! {}, quote! { [T] })]
    #[should_panic(expected = "unsupported Array")]
    #[case(quote! {}, quote! { [T; 4] })]
    #[should_panic(expected = "unsupported Ptr")]
    #[case(quote! {}, quote! { *const T })]
    #[should_panic(expected = "unsupported Reference")]
    #[case(quote! {}, quote! { &T })]
    #[should_panic(expected = "unsupported BareFn")]
    #[case(quote! {}, quote! { fn(T) -> T })]
    #[should_panic(expected = "unsupported Never")]
    #[case(quote! {}, quote! { ! })]
    #[should_panic(expected = "unsupported Tuple")]
    #[case(quote! {}, quote! { (T, U) })]
    #[should_panic(expected = "unsupported TraitObject")]
    #[case(quote! {}, quote! { T + U })]
    #[should_panic(expected = "unsupported Infer")]
    #[case(quote! { number }, quote! { _ })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: Type = syn::parse2(item).unwrap();
        assert_eq!(expected.to_string(), item.to_tstype().to_string());
    }
}
