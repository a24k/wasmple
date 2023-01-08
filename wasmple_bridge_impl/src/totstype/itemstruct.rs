use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemStruct;

use super::ToTsType;

impl ToTsType for ItemStruct {
    fn to_tstype(&self) -> TokenStream {
        let ident = &self.ident;
        let fields = self.fields.to_tstype();
        quote! { export type #ident = #fields; }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{ItemStruct, ToTsType, TokenStream};
    use quote::quote;

    #[rstest]
    #[case(quote! {
        export type TestStruct = { ptr: BufferPtr, num: number, str: string };
    }, quote! {
        struct TestStruct {
            ptr: BufferPtr,
            num: u32,
            str: String,
        }
    })]
    fn convert_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: ItemStruct = syn::parse2(item).unwrap();
        assert_eq!(expected.to_string(), item.to_tstype().to_string());
    }
}
