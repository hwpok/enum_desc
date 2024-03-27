use crate::enum_desc::enum_info::EnumInfo;
use proc_macro::TokenStream;

pub fn enum_desc_expand(input: TokenStream) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::Item);
    match do_expand(&item) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(item: &syn::Item) -> syn::Result<proc_macro2::TokenStream> {
    let ident = EnumInfo::get_enum_ident(item)?;
    let enum_info_vec = EnumInfo::parse(item)?;
    let get_code_ts = gen_get_code_ts(&enum_info_vec)?;
    let get_from_ts = gen_from_code_ts(&enum_info_vec)?;
    let get_desc_ts = gen_get_desc_ts(&enum_info_vec)?;
    let got_desc_ts = gen_got_desc_ts(&enum_info_vec)?;

    let token_stream_res = quote::quote!(
       impl #ident {

            #get_code_ts

            #get_desc_ts

            #get_from_ts

            #got_desc_ts
        }
    );
    Ok(token_stream_res)
}

fn gen_get_code_ts(enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { var, value, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
             Self::#var => #value,
        ));
    }
    let res = quote::quote!(
        pub fn to_code(&self) -> i16 {
            match self {
                #(#token_streams)*
            }
        }
    );
    Ok(res)
}

fn gen_get_desc_ts(enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { var, desc, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
             Self::#var => #desc.to_string(),
        ));
    }

    let res = quote::quote!(
        #[inline]
        pub fn get_desc(&self) -> String {
            match self {
                #(#token_streams)*
            }
        }
    );
    Ok(res)
}

fn gen_from_code_ts(enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { var, value, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
             #value => Some(Self::#var),
        ));
    }

    let res = quote::quote!(
        pub fn from_code(code: i16) -> Option<Self> {
            match code {
                #(#token_streams)*
                _ => None,
            }
        }
    );
    Ok(res)
}

fn gen_got_desc_ts(enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { var, desc, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
           Some(Self::#var) => #desc.to_string(),
        ));
    }

    let res = quote::quote!(
        pub fn got_desc(code: i16) -> String {
            match Self::from_code(code) {
                 #(#token_streams)*
                None => "".to_string(),
            }
        }
    );
    Ok(res)
}
