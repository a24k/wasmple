use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemEnum;

use super::ToTsType;

impl ToTsType for ItemEnum {
    fn to_tstype(&self) -> TokenStream {
        let ident = &self.ident;

        let variants = self.variants.iter().map(|variant| variant.to_tstype());

        //let fields = self.fields.to_tstype();
        quote! { export enum #ident { #(#variants),* } }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ItemEnum, ToTsType, TokenStream};
    use quote::quote;

    #[rstest]
    #[case(quote! {
        export enum T { I8, U8, I16, U16, I32, U32, I64, U64, F32, F64 }
    }, quote! {
        pub(super) enum T { I8, U8, I16, U16, I32, U32, I64, U64, F32, F64, }
    })]
    #[case(quote! {
        export enum LogLevel { Log, Debug, Info, Warn, Error }
    }, quote! {
        enum LogLevel { Log, Debug, Info, Warn, Error, }
    })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: ItemEnum = syn::parse2(item).unwrap();
        assert_eq!(expected.to_string(), item.to_tstype().to_string());
    }
}
