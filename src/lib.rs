use crate::enum_desc::enum_desc_expand::enum_desc_expand;
use crate::enum_trs::enum_trs_expand::enum_trs_expand;
use proc_macro::TokenStream;

mod enum_desc;
mod enum_trs;

#[proc_macro_derive(EnumDesc, attributes(info))]
pub fn enum_desc(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "i16")
}

#[proc_macro_derive(EnumDescI8, attributes(info))]
pub fn enum_desc_i8(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "i8")
}
#[proc_macro_derive(EnumDescU8, attributes(info))]
pub fn enum_desc_u8(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "u8")
}
#[proc_macro_derive(EnumDescI16, attributes(info))]
pub fn enum_desc_i16(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "i16")
}
#[proc_macro_derive(EnumDescU16, attributes(info))]
pub fn enum_desc_u16(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "u16")
}
#[proc_macro_derive(EnumDescI32, attributes(info))]
pub fn enum_desc_i32(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "i32")
}
#[proc_macro_derive(EnumDescU32, attributes(info))]
pub fn enum_desc_u32(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "u32")
}

#[proc_macro_derive(EnumDescI64, attributes(info))]
pub fn enum_desc_i64(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "i64")
}
#[proc_macro_derive(EnumDescU64, attributes(info))]
pub fn enum_desc_u64(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "u64")
}

#[proc_macro_derive(EnumDescISize, attributes(info))]
pub fn enum_desc_isize(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "isize")
}
#[proc_macro_derive(EnumDescUSize, attributes(info))]
pub fn enum_desc_usize(input: TokenStream) -> TokenStream {
    enum_desc_expand(input, "usize")
}

#[proc_macro_attribute]
pub fn enum_trs(attr: TokenStream, input: TokenStream) -> TokenStream {
    enum_trs_expand(attr, input)
}
