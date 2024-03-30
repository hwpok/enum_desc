use crate::enum_desc::enum_desc_expand::enum_desc_expand;
use crate::enum_trs::enum_trs_expand::enum_trs_expand;
use proc_macro::TokenStream;

mod enum_desc;
mod enum_trs;

#[proc_macro_derive(EnumDesc, attributes(info))]
pub fn enum_desc(input: TokenStream) -> TokenStream {
    enum_desc_expand(input)
}

#[proc_macro_attribute]
pub fn enum_trs(attr: TokenStream, input: TokenStream) -> TokenStream {
    enum_trs_expand(attr, input)
}
