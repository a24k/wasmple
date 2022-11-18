use proc_macro2::TokenStream;
use quote::quote;
use syn::TypePath;

use super::ToTsType;
use crate::unsupported;

impl ToTsType for TypePath {
    fn to_tstype_token_stream(&self) -> TokenStream {
        let segs = &self.path.segments;

        if segs.len() != 1 {
            unsupported!(self);
        }

        segs.last()
            .map_or(quote! {}, |seg| match seg.ident.to_string().as_str() {
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
                "Vec" => unsupported!(self),
                _ => {
                    let ident = &seg.ident;
                    quote! { #ident }
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ToTsType, TokenStream, TypePath};
    use quote::quote;

    #[rstest]
    #[should_panic(expected = "unsupported TypePath")]
    #[case(quote! {}, quote! { unknown::unknown })]
    #[should_panic(expected = "unsupported TypePath")]
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
    #[case(quote! { unknown }, quote! { unknown })]
    #[case(quote! { TestStruct }, quote! { TestStruct })]
    fn convert_typepath_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: TypePath = syn::parse2(item).unwrap();
        assert_eq!(
            expected.to_string(),
            item.to_tstype_token_stream().to_string()
        );
    }
}
