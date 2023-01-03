use proc_macro2::{Span, TokenStream};

#[derive(Debug)]
pub(crate) enum Error {
    /// An error generated internally by the odoo attribute/derive macro functions
    MacroError((String, Option<Span>)),

    /// A generic TokenStream "error"
    TokenStream(TokenStream),
}

impl From<TokenStream> for Error {
    fn from(ts: TokenStream) -> Self {
        Self::TokenStream(ts)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::MacroError((s, None))
    }
}

impl From<(String, Option<Span>)> for Error {
    fn from(value: (String, Option<Span>)) -> Self {
        Self::MacroError(value)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        s.to_string().into()
    }
}

impl From<(&str, Option<Span>)> for Error {
    fn from(value: (&str, Option<Span>)) -> Self {
        (value.0.to_string(), value.1).into()
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;
