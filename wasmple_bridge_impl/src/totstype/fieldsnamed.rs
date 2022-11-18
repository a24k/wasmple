use proc_macro2::TokenStream;
use quote::quote;
use syn::FieldsNamed;

use super::ToTsType;

impl ToTsType for FieldsNamed {
    fn to_tstype_token_stream(&self) -> TokenStream {
        let fields = self.named.iter().map(|field| {
            let ident = &field.ident;
            let ty = field.ty.to_tstype_token_stream();
            quote! { #ident: #ty }
        });

        quote! { { #(#fields),* } }
    }
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::{FieldsNamed, ToTsType, TokenStream};
    use quote::quote;

    #[rstest]
    #[case(quote! {
        { num: number, str: string }
    }, quote! {
        { num: u32, str: String, }
    })]
    #[should_panic(expected = "unsupported TypePath")]
    #[case(quote! {
        { length: number, str: string }
    }, quote! {
        { length: usize, str: Vec<String>, }
    })]
    fn convert_itemtype_to_tstype(#[case] expected: TokenStream, #[case] item: TokenStream) {
        let item: FieldsNamed = syn::parse2(item).unwrap();
        assert_eq!(
            expected.to_string(),
            item.to_tstype_token_stream().to_string()
        );
    }
}
