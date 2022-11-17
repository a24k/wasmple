use proc_macro2::TokenStream;
use quote::quote;
use syn::TypePath;

use super::ToTsType;

impl ToTsType for TypePath {
    fn to_tstype_token_stream(&self) -> TokenStream {
        let segs = &self.path.segments;

        if segs.len() != 1 {
            return quote! {};
        }

        segs.first().map_or(quote! {}, |segment| {
            match segment.ident.to_string().as_str() {
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
                _ => quote! {},
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
    #[case(quote! {}, quote! { unknown })]
    #[case(quote! {}, quote! { unknown::unknown })]
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
    fn convert_typepath_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: TypePath = syn::parse2(item).unwrap();
        assert_eq!(
            expected.to_string(),
            item.to_tstype_token_stream().to_string()
        );
    }
}
