mod enum_desc;
use crate::enum_desc::enum_expand::enum_desc_expand;
use proc_macro::TokenStream;

#[proc_macro_derive(EnumDesc, attributes(info))]
pub fn enum_desc(input: TokenStream) -> TokenStream {
    enum_desc_expand(input)
}
