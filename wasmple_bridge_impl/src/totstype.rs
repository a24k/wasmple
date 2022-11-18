mod fields;
mod fieldsnamed;
mod item;
mod itemstruct;
mod itemtype;
mod r#type;
mod typepath;

use proc_macro2::TokenStream;

pub(super) trait ToTsType {
    fn to_tstype_token_stream(&self) -> TokenStream;
}
