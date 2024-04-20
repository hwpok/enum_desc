use crate::enum_desc::enum_info::EnumInfo;
use proc_macro::TokenStream;
use syn::spanned::Spanned;

pub(crate) fn enum_desc_expand(input: TokenStream, enum_var_tp_str: &str) -> TokenStream {
    let item = syn::parse_macro_input!(input as syn::Item);
    let enum_var_tp = syn::Ident::new(enum_var_tp_str,  item.span());
    match do_expand(&item, enum_var_tp) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(item: &syn::Item, enum_var_tp: syn::Ident) -> syn::Result<proc_macro2::TokenStream> {
    let ident = EnumInfo::get_enum_ident(item)?;
    let enum_info_vec = EnumInfo::parse(item)?;
    let get_code_ts = gen_get_code_ts(&enum_var_tp, &enum_info_vec)?;
    let get_from_ts = gen_from_code_ts(&enum_var_tp, &enum_info_vec)?;
    let get_desc_ts = gen_get_desc_ts(&enum_info_vec)?;
    let got_desc_ts = gen_get_desc_from_code_ts(&enum_var_tp, &enum_info_vec)?;
    //let get_multi_desc_ts = gen_get_multi_desc_ts(&enum_info_vec)?;
    let get_bitflag_desc_ts = gen_get_desc_from_bitflag_ts(&enum_var_tp, &enum_info_vec)?;
    let get_val_desc_pairs_ts = gen_get_val_desc_pairs(&enum_var_tp, &enum_info_vec, )?;

    let token_stream_res = quote::quote!(
       impl #ident {

            #get_code_ts

            #get_desc_ts

            #get_from_ts

            #got_desc_ts

            #get_val_desc_pairs_ts

            //#get_multi_desc_ts

            #get_bitflag_desc_ts
        }
    );
    Ok(token_stream_res)
}

fn gen_get_code_ts(enum_var_tp: &syn::Ident, enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { var, value, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
             Self::#var => #value,
        ));
    }
    let res = quote::quote!(
        pub fn to_code(&self) -> #enum_var_tp {
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

fn gen_from_code_ts(enum_var_tp: &syn::Ident, enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { var, value, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
             #value => Some(Self::#var),
        ));
    }

    let res = quote::quote!(
        pub fn from_code(code: #enum_var_tp) -> Option<Self> {
            match code {
                #(#token_streams)*
                _ => None,
            }
        }
    );
    Ok(res)
}

fn gen_get_desc_from_code_ts(enum_var_tp: &syn::Ident, enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { var, desc, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
           Some(Self::#var) => #desc.to_string(),
        ));
    }

    let res = quote::quote!(
        pub fn get_desc_from_code(code: #enum_var_tp) -> String {
            match Self::from_code(code) {
                 #(#token_streams)*
                None => "".to_string(),
            }
        }
    );
    Ok(res)
}

/*
fn gen_get_multi_desc_ts(enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { value, desc, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
           #value => desc_vec.push(#desc),
        ));
    }

    let res = quote::quote!(
        pub fn get_multi_desc(codes: Vec<isize>) -> String {
            let mut desc_vec = vec![];
            for code in codes {
                match code {
                   #(#token_streams)*
                    _ => {}
                }
            }
            return desc_vec.join(" | ").to_string()
        }
    );
    Ok(res)
}
*/


fn gen_get_desc_from_bitflag_ts(enum_var_tp: &syn::Ident, enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { value, desc, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
           if code & #value != 0 { desc_vec.push(#desc)};
        ));
    }

    let res = quote::quote!(
        pub fn get_desc_from_bitflag(code: #enum_var_tp) -> String {
            let mut desc_vec = vec![];
            #(#token_streams)*
            return desc_vec.join(" | ").to_string()
        }
    );
    Ok(res)
}

fn gen_get_val_desc_pairs(enum_var_tp: &syn::Ident, enum_info_vec: &Vec<EnumInfo>) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for EnumInfo { value, desc, .. } in enum_info_vec.iter() {
        token_streams.push(quote::quote!(
           pairs.push((#value, #desc.to_string()));
        ));
    }

    let res = quote::quote!(
        pub fn get_val_desc_pairs() -> Vec<(#enum_var_tp, String)> {
            let mut pairs = vec![];
             #(#token_streams)*
            pairs
        }
    );
    Ok(res)
}