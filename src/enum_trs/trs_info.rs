use std::collections::HashMap;
use syn::ItemStruct;

#[derive(Debug)]
pub(crate) struct TrsInfo {
    pub field: syn::Ident,
    pub field_generic: bool,
    pub bit_flag: bool,
    pub des_field: syn::Field,
    pub des_enum: syn::Path,
}

impl TrsInfo {
    pub(crate) fn parse(derive_input: &ItemStruct, attr: String) -> syn::Result<Vec<TrsInfo>> {
        let field_enum_entry_map: HashMap<String, (String, bool)> = attr
            .split(',')
            .filter(|s| !s.trim().is_empty())
            .map(|s| {
                let bit_flag = s.to_string().contains("&");
                if let Some((k, v)) = s.split_once(if bit_flag {'&'} else {'='}) {
                    (k.trim().to_string(), (v.trim().to_string(), bit_flag))
                } else {
                    // Use the Sentinel Pattern to simplify the algorithm
                    ("sentinelHuiwanpengAtChengdu".to_string(), ("nothing".to_string(), true))
                }
            })
            .collect();

        if let syn::Fields::Named(syn::FieldsNamed { ref named, .. }) = derive_input.fields {
            let mut trs_info_vec: Vec<TrsInfo> = Vec::new();
            for field in named {
                if let Some(ident) = &field.ident {
                    if field_enum_entry_map.contains_key(ident.to_string().as_str()) {
                        let (enum_name, bit_flag) = field_enum_entry_map.get(ident.to_string().as_str()).unwrap();
                        let (is_option, field_ty) = Self::get_file_type(&field.ty);
                        let field_ty_ident = Self::get_file_type_ident(&field_ty);
                        if let Ok(path) = syn::parse_str(enum_name) {
                            let types = vec!["i8", "u8", "i16", "u16", "i32", "u32", "i64", "u64", "isize","usize"];
                            let file_type = field_ty_ident
                                .map(|ident| ident.to_string())
                                .unwrap_or_default();
                            if types.contains(&file_type.as_str()) {
                                trs_info_vec.push(TrsInfo {
                                    field: ident.clone(),
                                    field_generic: is_option,
                                    bit_flag: bit_flag.clone(),
                                    des_field: syn::Field {
                                        attrs: Vec::new(),
                                        vis: syn::parse_quote!(pub),
                                        mutability: syn::FieldMutability::None,
                                        ident: Some(syn::Ident::new(
                                            format!("{}_desc", ident.to_string()).as_str(),
                                            ident.span(),
                                        )),
                                        colon_token: None,
                                        ty: syn::parse_quote!(String),
                                    },
                                    des_enum: path,
                                });
                            } else {
                                return Err(syn::Error::new_spanned(field, format!("Expected one type: {}", types.join(", "))));
                            }
                        } else {
                            return Err(syn::Error::new_spanned(
                                field,
                                format!("Expected enum path error: {}", enum_name),
                            ));
                        }
                    }
                }
            }
            return Ok(trs_info_vec);
        };

        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected enum",
        ))
    }

    fn get_file_type(ty: &syn::Type) -> (bool, syn::Type) {
        if let syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) = ty
        {
            if let Some(seg) = segments.last() {
                if seg.ident.to_string() == "Option" {
                    if let syn::PathArguments::AngleBracketed(
                        syn::AngleBracketedGenericArguments { args, .. },
                    ) = &seg.arguments
                    {
                        if let Some(syn::GenericArgument::Type(inner_type)) = args.first() {
                            return (true, inner_type.clone());
                        }
                    }
                }
            }
        }
        return (false, ty.clone());
    }

    fn get_file_type_ident(ty: &syn::Type) -> Option<syn::Ident> {
        if let syn::Type::Path(syn::TypePath {
            path: syn::Path { segments, .. },
            ..
        }) = ty
        {
            if let Some(seg) = segments.last() {
                return Some(seg.ident.clone());
            }
        }
        None
    }
}
