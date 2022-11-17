use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn wasmple_bridge(attr: TokenStream, item: TokenStream) -> TokenStream {
    wasmple_bridge_impl::wasmple_bridge_impl(attr.into(), item.into()).into()
}
