use crate::{Error, Result};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span, TokenStream as TokenStream2};
use quote::{quote, quote_spanned};
use syn::ext::IdentExt;
use syn::{Fields, FieldsNamed, ItemStruct, Lit, LitStr, Meta, MetaNameValue, Token};

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

/// Custom arguments type
///
/// This type implements [`syn::parse::Parse`], and will convert the macro
/// arguments into a format useable by this crate. Note that we can't use
/// [`syn::AttributeArgs`] because that types' parse impl doesn't support
/// arrays (e.g. `test = ["list", "of", "literals"]). We also don't need
/// to support paths as arg keys (for now), so the returned struct can be
/// simpler and easier to work with on the macro impl side
#[derive(Debug)]
pub(crate) struct MacroArguments {
    inner: Vec<Arg>,
}

#[derive(Debug)]
pub(crate) struct Arg {
    pub(crate) key: String,
    pub(crate) span: Span,
    pub(crate) value: ArgValue,
}

#[derive(Debug)]
pub(crate) enum ArgValue {
    Lit(Lit),
    Array(Vec<String>),
}

impl IntoIterator for MacroArguments {
    type IntoIter = <Vec<Arg> as IntoIterator>::IntoIter;
    type Item = Arg;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl TryFrom<ArgValue> for String {
    type Error = Error;
    fn try_from(value: ArgValue) -> std::result::Result<String, Self::Error> {
        match value {
            ArgValue::Lit(Lit::Str(lit)) => Ok(lit.value()),
            _ => Err("expected LitStr, got something else".into()),
        }
    }
}
impl TryFrom<ArgValue> for bool {
    type Error = Error;
    fn try_from(value: ArgValue) -> std::result::Result<bool, Self::Error> {
        match value {
            ArgValue::Lit(Lit::Bool(lit)) => Ok(lit.value()),
            _ => Err("expected LitBool, got something else".into()),
        }
    }
}
impl TryFrom<ArgValue> for Vec<String> {
    type Error = Error;
    fn try_from(value: ArgValue) -> std::result::Result<Vec<String>, Self::Error> {
        match value {
            ArgValue::Array(val) => Ok(val),
            _ => Err("expected LitBool, got something else".into()),
        }
    }
}

impl syn::parse::Parse for MacroArguments {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut inner = Vec::new();

        while input.peek(Ident::peek_any) {
            inner.push(input.parse()?);
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { inner })
    }
}

impl syn::parse::Parse for Arg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let key = input.parse::<Ident>()?;
        let span = key.span();
        let key = key.to_string();
        input.parse::<Token![=]>()?;
        let value = input.parse()?;
        Ok(Self { key, span, value })
    }
}

impl syn::parse::Parse for ArgValue {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::token::Bracket) {
            // the grouped `path = ["group", "of", "literals"]` format
            let content;
            let mut values = Vec::new();
            syn::bracketed!(content in input);
            while content.peek(Lit) {
                values.push(content.parse::<LitStr>()?.value());
                if content.peek(Token![,]) {
                    content.parse::<Token![,]>()?;
                }
            }

            Ok(ArgValue::Array(values))
        } else if input.peek(Lit) {
            // standard `path = "literal"` format
            input.parse().map(ArgValue::Lit)
        } else {
            Err(input.error("expected identifier or literal"))
        }
    }
}
