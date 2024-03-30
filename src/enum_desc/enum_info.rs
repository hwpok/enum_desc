#[derive(Debug)]
pub(crate) struct EnumInfo {
    pub var: syn::Ident,
    pub value: syn::LitInt,
    pub desc: proc_macro2::Literal,
}

impl EnumInfo {
    pub(crate) fn get_enum_ident(item: &syn::Item) -> syn::Result<syn::Ident> {
        if let syn::Item::Enum(item_enum) = item {
            return Ok(item_enum.ident.clone());
        }
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected enum",
        ))
    }
    pub(crate) fn parse(item: &syn::Item) -> syn::Result<Vec<EnumInfo>> {
        if let syn::Item::Enum(item_enum) = item {
            let mut enum_infos = Vec::new();
            for (idx, variant) in item_enum.variants.iter().enumerate() {
                let syn::Variant {
                    ident,
                    discriminant,
                    attrs,
                    ..
                } = variant;

                let mut enum_info = Self {
                    var: ident.clone(),
                    value: syn::LitInt::new(idx.to_string().as_str(), ident.span()),
                    desc: proc_macro2::Literal::string(""),
                };

                if let Some((_, syn::Expr::Lit(expr_lit))) = discriminant {
                    if let syn::Lit::Int(expr_lit) = &expr_lit.lit {
                        enum_info.value = expr_lit.clone();
                    }
                } else {
                    return Err(syn::Error::new_spanned(
                        variant,
                        "Enumerations must be set explicitly",
                    ));
                }

                if let Some(syn::Attribute { meta, .. }) = attrs.first() {
                    if let syn::Meta::List(syn::MetaList { tokens, .. }) = meta {
                        /*
                        let buff = syn::buffer::TokenBuffer::new2(tokens.clone());
                        let mut cursor = buff.begin();
                        while let Some((token_tree, next_cur)) = cursor.token_tree() {
                            cursor = next_cur;
                            if let proc_macro2::TokenTree::Literal(literal) = token_tree {
                                enum_info.desc = literal;
                            }
                        }
                        */
                        for token_tree in tokens.clone().into_iter() {
                            if let proc_macro2::TokenTree::Literal(literal) = token_tree {
                                enum_info.desc = literal;
                            }
                        }
                    }
                }
                enum_infos.push(enum_info);
            }
            return Ok(enum_infos);
        }
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Expected enum",
        ))
    }
}
