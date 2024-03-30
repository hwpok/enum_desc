use crate::enum_trs::trs_info::TrsInfo;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemStruct};

pub(crate) fn enum_trs_expand(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_copy = input.clone();
    let mut derive_input = parse_macro_input!(input_copy as DeriveInput);
    let item_struct = parse_macro_input!(input as ItemStruct);
    match do_expand(attr, &item_struct, &mut derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(
    attr: TokenStream,
    item_struct: &ItemStruct,
    derive_input: &mut DeriveInput,
) -> syn::Result<proc_macro2::TokenStream> {
    let trs_info_vec = TrsInfo::parse(&item_struct, attr.to_string())?;

    // append description field into struct
    let _ = build_append_desc_field_ts(derive_input, &trs_info_vec)?;

    // append 'translate_enums' function to struct
    let struct_ident = &item_struct.ident;
    let desc_field_setting_ts = build_desc_field_setting_ts(&trs_info_vec);
    let res = quote::quote!(
        #derive_input

        impl #struct_ident {
            pub fn translate_enums(&mut self) {
                #(#desc_field_setting_ts)*
            }
        }
    );

    Ok(res)
}

fn build_append_desc_field_ts(
    derive_input: &mut DeriveInput,
    trs_info_vec: &Vec<TrsInfo>,
) -> syn::Result<String> {
    match &mut derive_input.data {
        syn::Data::Struct(ref mut s) => {
            if let syn::Fields::Named(ref mut fields) = s.fields {
                for field in trs_info_vec.iter() {
                    fields.named.push(field.des_field.clone());
                }
            } else {
                return Err(syn::Error::new_spanned(
                    derive_input,
                    "Expected a struct with named fields",
                ));
            }
        }
        _ => {
            return Err(syn::Error::new_spanned(derive_input, "Expected a struct"));
        }
    }
    Ok("success".to_string())
}

fn build_desc_field_setting_ts(trs_info_vec: &Vec<TrsInfo>) -> Vec<proc_macro2::TokenStream> {
    // make init field item
    let mut init_item_ts_vec: Vec<proc_macro2::TokenStream> = Vec::new();
    for TrsInfo {
        field,
        field_generic,
        des_field,
        des_enum,
    } in trs_info_vec.iter()
    {
        let desc_field_ident = des_field.ident.clone().unwrap();

        // build enum path
        let segments = &des_enum.segments;
        let mut path_idents = quote::quote! {};
        for (idx, segment) in segments.iter().enumerate() {
            let ident = &segment.ident;
            if idx == 0 {
                path_idents = quote::quote! { #ident };
            } else {
                path_idents = quote::quote! { #path_idents :: #ident };
            }
        }

        // set desc field's value
        if *field_generic {
            init_item_ts_vec.push(quote::quote!(
              self.#desc_field_ident = self.#field.map_or_else(|| String::new(), |#field|  #path_idents::got_desc(#field));
            ));
        } else {
            init_item_ts_vec.push(quote::quote!(
              self.#desc_field_ident = #path_idents::got_desc(self.#field);
            ));
        }
    }
    init_item_ts_vec
}
