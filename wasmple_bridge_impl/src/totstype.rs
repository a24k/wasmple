mod fields;
mod fieldsnamed;
mod fnarg;
mod item;
mod itemenum;
mod itemfn;
mod itemstruct;
mod itemtype;
mod pat;
mod r#type;
mod typepath;
mod variant;

use proc_macro2::TokenStream;

pub(super) trait ToTsType {
    fn to_tstype(&self) -> TokenStream;
}
