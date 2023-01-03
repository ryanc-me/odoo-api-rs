use std::collections::HashSet;

use crate::{Error, Result};
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{AttributeArgs, Fields, FieldsNamed, ItemStruct, Lit, Meta, MetaNameValue, NestedMeta};

/// Wrapper type that implements a custom [`syn::parse::Parse`]
pub(crate) struct ItemStructNamed {
    pub item: ItemStruct,
    pub fields: FieldsNamed,
    pub doc_head: String,
}

impl syn::parse::Parse for ItemStructNamed {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let item = ItemStruct::parse(input);
        match &item {
            Ok(ItemStruct {
                fields: Fields::Named(fields),
                ..
            }) => {
                let item = item.clone().unwrap();
                let mut doc_head = String::new();
                for attr in &item.attrs {
                    if let Meta::NameValue(MetaNameValue {
                        path,
                        lit: Lit::Str(lit),
                        ..
                    }) = attr.parse_meta()?
                    {
                        if let Some(segment) = path.segments.first() {
                            if segment.ident == "doc" {
                                doc_head = lit.value();
                                break;
                            }
                        }
                    }
                }

                Ok(Self {
                    item,
                    fields: fields.clone(),
                    doc_head,
                })
            }
            _ => Err(syn::Error::new(
                Span::mixed_site(),
                "This macro must be applied to a struct with named fields",
            )),
        }
    }
}

impl quote::ToTokens for ItemStructNamed {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.item.to_tokens(tokens)
    }
}

/// Helper to parse [`crate::Result`] into [`TokenStream`]
pub(crate) fn parse_result(result: Result<TokenStream2>) -> TokenStream {
    match result {
        Ok(ts) => ts,
        Err(err) => match err {
            Error::TokenStream(ts) => ts,
            Error::MacroError(s) => {
                let message = s.0;
                let span = s.1;
                match span {
                    Some(span) => quote_spanned! {span=> compile_error!(#message);},
                    None => quote! {compile_error!(#message);},
                }
            }
        },
    }
    .into()
}

/// Result for the [`parse_args`] function (see there for info)
type MacroArgs = Vec<(String, Lit, Span)>;

/// Parse an [`AttributeArgs`] into a `(key, value, span)` tuple
///
/// This makes it much more ergonomic to match key and value(type) pairs
pub(crate) fn parse_args(args: AttributeArgs) -> Result<MacroArgs> {
    let err_string = "Unexpected input. The macro input should be a list of name-value pairs (e.g., `method = \"execute\"`)";
    let mut seen = HashSet::new();
    let mut result = Vec::new();

    for item in args {
        match &item {
            NestedMeta::Meta(Meta::NameValue(MetaNameValue { path, lit, .. })) => {
                match path.clone().segments.first() {
                    Some(segment) => {
                        let key = segment.ident.clone().to_string();
                        if seen.contains(&key) {
                            Err((format!("Duplicate key `{}`", key), Some(item.span())))?
                        }
                        result.push((key.clone(), lit.clone(), item.span()));
                        seen.insert(key);
                    }
                    None => Err((err_string, Some(item.span())))?,
                }
            }

            // all other cases are errors
            _ => Err((err_string, Some(item.span())))?,
        }
    }

    Ok(result)
}
