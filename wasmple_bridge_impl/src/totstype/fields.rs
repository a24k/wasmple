use proc_macro2::TokenStream;
use syn::Fields;

use crate::unsupported;
use super::ToTsType;

impl ToTsType for Fields {
    fn to_tstype_token_stream(&self) -> TokenStream {
        match self {
            Fields::Named(fields) => fields.to_tstype_token_stream(),
            _ => unsupported!(self),
        }
    }
}
