use proc_macro2::TokenStream;
use quote::quote;
use syn::TypePath;

use super::ToTsType;
use crate::unsupported;

impl ToTsType for TypePath {
    fn to_tstype_token_stream(&self) -> TokenStream {
        match self.path.segments.last() {
            None => unsupported!(self),
            Some(seg) => {
                match seg.ident.to_string().as_str() {
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
                    "Vec" => unsupported!(self), // to be recognized as array[]
                    _ => {
                        let ident = &seg.ident;
                        quote! { #ident }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ToTsType, TokenStream, TypePath};
    use quote::quote;

    #[rstest]
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
    #[case(quote! { unknown }, quote! { unknown::unknown })]
    #[case(quote! { number }, quote! { usize::usize })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: TypePath = syn::parse2(item).unwrap();
        assert_eq!(
            expected.to_string(),
            item.to_tstype_token_stream().to_string()
        );
    }
}
