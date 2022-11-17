use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(JsonConvertee)]
pub fn derive_json_convertee(item: TokenStream) -> TokenStream {
    let item: DeriveInput = syn::parse(item).unwrap();

    let ident = item.ident;

    let impl_json_convertee = quote! {
        impl JsonConvertee for #ident {}
    };

    impl_json_convertee.into()
}
