mod fields;
mod fieldsnamed;
mod fnarg;
mod item;
mod itemfn;
mod itemstruct;
mod itemtype;
mod pat;
mod r#type;
mod typepath;

use proc_macro2::TokenStream;

pub(super) trait ToTsType {
    fn to_tstype(&self) -> TokenStream;
}
