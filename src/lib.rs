use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(EnumDesc, attributes(info))]
pub fn do_enhance_enum_expand(input: TokenStream) -> TokenStream {
    let derive_input = parse_macro_input!(input as DeriveInput);
    match do_expand(&derive_input) {
        Ok(token_stream) => token_stream.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn do_expand(derive_input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
    let DeriveInput { ident, .. } = derive_input;
    let enum_info_vec = get_enum_data(derive_input)?;

    let get_code_ts = gen_get_code_token_stream(&enum_info_vec)?;
    let get_from_ts = gen_from_code_token_stream(&enum_info_vec)?;
    let get_desc_ts = gen_get_desc_token_stream(&enum_info_vec)?;
    let get_desc2_ts = gen_get_desc2_token_stream(&enum_info_vec)?;

    let token_stream_res = quote!(
       impl #ident {
            #get_code_ts

            #get_desc_ts

            #get_from_ts

            #get_desc2_ts
        }
    );
    Ok(token_stream_res)
}

fn get_enum_data(
    derive_input: &DeriveInput,
) -> syn::Result<Vec<(syn::Ident, syn::LitInt, proc_macro2::Literal)>> {
    let mut datas = Vec::new();
    if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &derive_input.data {
        for (index, variant) in variants.iter().enumerate() {
            let syn::Variant {
                ident,
                discriminant,
                attrs,
                ..
            } = variant;

            let mut data_tuple: (syn::Ident, syn::LitInt, proc_macro2::Literal) = (
                ident.clone(),
                syn::LitInt::new(index.to_string().as_str(), ident.span()),
                proc_macro2::Literal::string("desc"),
            );

            if let Some((_, syn::Expr::Lit(expr_lit))) = discriminant {
                if let syn::Lit::Int(expr_lit) = &expr_lit.lit {
                    data_tuple.1 = expr_lit.clone();
                }
            } else {
                return Err(syn::Error::new_spanned(
                    variant,
                    "Enumerations must be set explicitly",
                ));
            }

            if let Some(syn::Attribute { meta, .. }) = attrs.first() {
                if let syn::Meta::List(syn::MetaList { tokens, .. }) = meta {
                    for token_tree in tokens.clone().into_iter() {
                        if let proc_macro2::TokenTree::Literal(literal) = token_tree {
                            data_tuple.2 = literal;
                        }
                    }
                }
            }
            datas.push(data_tuple);
        }
    }
    return Ok(datas);
}

fn gen_get_code_token_stream(
    enum_info_vec: &Vec<(syn::Ident, syn::LitInt, proc_macro2::Literal)>,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for (field, val, _desc) in enum_info_vec.iter() {
        token_streams.push(quote!(
             Self::#field => #val,
        ));
    }

    let res = quote!(
        pub fn to_code(&self) -> i16 {
            match self {
                #(#token_streams)*
            }
        }
    );
    Ok(res)
}

fn gen_get_desc_token_stream(
    enum_info_vec: &Vec<(syn::Ident, syn::LitInt, proc_macro2::Literal)>,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for (field, _val, desc) in enum_info_vec.iter() {
        token_streams.push(quote!(
             Self::#field => #desc,
        ));
    }

    let res = quote!(
        #[inline]
        pub fn get_desc(&self) -> &'static str {
            match self {
                #(#token_streams)*
            }
        }
    );
    Ok(res)
}

fn gen_from_code_token_stream(
    enum_info_vec: &Vec<(syn::Ident, syn::LitInt, proc_macro2::Literal)>,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for (field, val, _desc) in enum_info_vec.iter() {
        token_streams.push(quote!(
             #val => Some(Self::#field),
        ));
    }

    let res = quote!(
        pub fn from_code(code: i16) -> Option<Self> {
            match code {
                #(#token_streams)*
                _ => None,
            }
        }
    );
    Ok(res)
}

fn gen_get_desc2_token_stream(
    enum_info_vec: &Vec<(syn::Ident, syn::LitInt, proc_macro2::Literal)>,
) -> syn::Result<proc_macro2::TokenStream> {
    let mut token_streams = Vec::new();
    for (field, _val, desc) in enum_info_vec.iter() {
        token_streams.push(quote!(
           Some(Self::#field) => #desc,
        ));
    }

    let res = quote!(
        pub fn get_desc2(code: i16) -> &'static str {
            match Self::from_code(code) {
                 #(#token_streams)*
                None => "",
            }
        }
    );
    Ok(res)
}
