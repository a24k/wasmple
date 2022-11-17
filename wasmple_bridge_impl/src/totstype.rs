mod item;
mod itemtype;
mod typepath;

use proc_macro2::TokenStream;

pub(super) trait ToTsType {
    fn to_tstype_token_stream(&self) -> TokenStream;
}
