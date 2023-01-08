use proc_macro2::TokenStream;
use syn::Fields;

use super::ToTsType;
use crate::unsupported;

impl ToTsType for Fields {
    fn to_tstype(&self) -> TokenStream {
        match self {
            Fields::Named(fields) => fields.to_tstype(),
            _ => unsupported!(self),
        }
    }
}
