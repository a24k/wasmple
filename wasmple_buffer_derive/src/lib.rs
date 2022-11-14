use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(JsonConvertee)]
pub fn derive_json_convertee(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();

    let ident = input.ident;

    let impl_json_convertee = quote! {
        impl JsonConvertee for #ident {}
    };

    impl_json_convertee.into()
}
