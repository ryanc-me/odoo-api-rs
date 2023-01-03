use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use crate::Result;

//TODO
pub(crate) fn serialize_tuple(input: TokenStream) -> Result<TokenStream2> {
    Ok(input.into())
}
