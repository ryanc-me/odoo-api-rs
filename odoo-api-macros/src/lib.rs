//! Helper macros for the `odoo_api` crate
//!
//! See the [`odoo_api`](https://crates.io/crates/odoo-api) crate.

use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{Field, Ident, ItemStruct, Lit, LitStr, Meta, MetaNameValue, Token, Type};

#[derive(Debug)]
enum Error {
    MacroError(String),

    TokenStream(TokenStream),
}

impl From<TokenStream> for Error {
    fn from(ts: TokenStream) -> Self {
        Self::TokenStream(ts)
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Self::MacroError(s)
    }
}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Self::MacroError(s.to_string())
    }
}

type Result<T> = std::result::Result<T, Error>;

/// A struct representing args to the `#[odoo_api_request(service, route)]` macro
struct OdooApiRequestArgs {
    /// Service: "common", "object", or "db"
    service: String,

    /// Method: THe method name, e.g., "execute_kw" or "create_database"
    method: String,

    /// Description of the API method
    description: String
}

impl Parse for OdooApiRequestArgs {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let service: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let method: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let description: LitStr = input.parse()?;

        Ok(OdooApiRequestArgs {
            service: service.value(),
            method: method.value(),
            description: description.value(),
        })
    }
}

#[proc_macro_attribute]
pub fn odoo_api_request(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    match odoo_api_request_impl(args, input) {
        Ok(ts) => ts,
        Err(err) => match err {
            Error::TokenStream(ts) => ts,
            Error::MacroError(s) => {
                quote!(compile_error!(#s))
            }
        },
    }
    .into()
}

fn odoo_api_request_impl(
    args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> Result<TokenStream> {
    // parse args and input (ensuring the macro was applied to a struct)
    let args: OdooApiRequestArgs = syn::parse(args).map_err(|e| e.to_compile_error())?;
    let input: ItemStruct = syn::parse(input).map_err(|e| e.to_compile_error())?;

    // make sure the struct has named fields, then parse
    let fields: Vec<Field> = match &input.fields {
        syn::Fields::Named(fields) => fields.named.clone().into_iter().collect(),
        _ => {
            let span = Span::call_site();
            return Err(quote_spanned! {span=>
                compile_error!("expected a struct with named fields")
            }
            .into());
        }
    };

    // make sure the struct doesn't have any generic params
    if !input.generics.params.is_empty() {
        let span = input.ident.span();
        return Err(quote_spanned! {span=>
            compile_error!("a struct tagged with `odoo_api_request` cannot have generic params");
        }
        .into());
    }

    // fetch the struct name (and some variations)
    let name_struct = input.ident.to_string();
    let name_response = format!("{}Response", &name_struct);
    let name_fn = name_struct.to_case(Case::Snake);
    let ident_struct = &input.ident;
    let ident_response = Ident::new(&name_response, Span::call_site());
    let ident_fn = Ident::new(&name_fn, Span::call_site());

    // extract doc comments from the original struct
    let doc = input
        .attrs
        .iter()
        .flat_map(|attr| {
            if !attr.path.is_ident("doc") {
                return None;
            }
            match attr.parse_meta() {
                Ok(Meta::NameValue(MetaNameValue {
                    lit: Lit::Str(lit_str),
                    ..
                })) => Some(lit_str.value()),
                _ => None,
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    let doc_base = format!("\
        {}\n\n\
        **Service**: `{}`  \n\
        **Method**: `{}`  \n\
        **Request**: [`{}`](crate::jsonrpc::types::{})  \n\
        **Response**: [`{}`](crate::jsonrpc::types::{})\n\n",
        &args.description,
        &args.service,
        &args.method,
        &name_struct,&name_struct,
        &name_response,&name_response
    );
    let doc = format!("{}{}",
        &doc_base,
        &doc
    );

    // parse the field names & types
    let mut has_db = false;
    let mut fields_args = Vec::<TokenStream>::new();
    let mut fields_assigns = Vec::<TokenStream>::new();
    let mut fields_call = Vec::<TokenStream>::new();
    for field in &fields {
        if let Type::Path(path) = &field.ty {
            if let Some(ident) = &field.ident {
                if ident == "db" {
                    has_db = true
                }
                let type_string = path.clone().into_token_stream().to_string();
                if type_string == "String" {
                    // rather than requiring full Strings, we'll accept &str and convert
                    fields_args.push(quote!(#ident: &str));
                    fields_assigns.push(quote!(#ident: #ident.to_string()));
                    fields_call.push(quote!(#ident));
                } else if type_string == "Vec < Value >" || type_string == "Map < String, Value >" {
                    fields_args.push(quote!(#ident: Value));
                    fields_assigns.push(quote!(#ident: ::serde_json::from_value(#ident)?));
                    fields_call.push(quote!(#ident));
                } else {
                    fields_args.push(quote!(#ident: #path));
                    fields_assigns.push(quote!(#ident: #ident));
                    fields_call.push(quote!(#ident));
                }
            }
        }
    }
    let mut fields_args2 = fields_args.clone();
    if !has_db {
        fields_args2.insert(0, quote!(db: &str))
    }

    // finally, generate the TokenStreams
    let out_api_method_impl = generate_api_method_impl(ident_struct, &ident_response, &args)?;
    let out_serialize_impl = generate_serialize_impl(ident_struct, &fields)?;
    let out_try_from_impl = generate_try_from_impl(&ident_response)?;
    let out_call = generate_call(ident_struct, &ident_fn, &fields_args, &fields_assigns, &doc)?;
    let out_call_async = generate_call_async(
        ident_struct,
        &ident_response,
        &ident_fn,
        &fields_args2,
        &fields_call,
        &doc_base,
        &args,
    )?;
    let out_call_blocking = generate_call_blocking(
        ident_struct,
        &ident_response,
        &ident_fn,
        &fields_args2,
        &fields_call,
        &doc_base,
        &args,
    )?;

    Ok(quote!(
        #[doc=#doc_base]
        #input
        #out_api_method_impl
        #out_serialize_impl
        #out_try_from_impl
        #out_call
        #out_call_async
        #out_call_blocking
    ))
}

fn generate_call(
    ident_struct: &Ident,
    ident_fn: &Ident,
    fields_args: &Vec<TokenStream>,
    fields_assigns: &Vec<TokenStream>,
    doc: &str,
) -> Result<TokenStream> {
    Ok(quote! {
        #[doc=#doc]
        #[allow(clippy::too_many_arguments)]
        pub fn #ident_fn(#(#fields_args),*) -> super::Result<super::OdooApiRequest<#ident_struct>> {
            #[cfg(not(test))]
            let id = {
                use ::rand::{Rng, thread_rng};
                let mut rng = thread_rng();
                rng.gen_range(1..10000)
            };

            #[cfg(test)]
            let id = 1000;

            Ok(super::OdooApiRequest {
                version: super::JsonRpcVersion::V2,
                method: super::JsonRpcMethod::Call,
                id: id,
                params: super::JsonRpcRequestParams {
                    args: #ident_struct {
                        #(
                            #fields_assigns,
                        )*
                    }
                }
            })
        }
    })
}

fn generate_call_async(
    ident_struct: &Ident,
    ident_response: &Ident,
    ident_fn: &Ident,
    fields_args: &Vec<TokenStream>,
    fields_call: &Vec<TokenStream>,
    doc_base: &str,
    args: &OdooApiRequestArgs,
) -> Result<TokenStream> {
    let name_fn_async = format!("{}_async", &ident_fn.to_string());
    let ident_fn_async = Ident::new(&name_fn_async, Span::call_site());
    let doc = format!("\
        {}\
        See the [`types::{}::{}`](crate::jsonrpc::types::{}::{}) function for \
        more information on this API method call, or the [`async`](crate::jsonrpc::asynch) \
        module for more information on making async requests.",
        doc_base,
        &args.service, &args.method,
        &args.service, &args.method,
    );

    Ok(quote!(
        pub(crate) mod #ident_fn_async {
            use serde_json::{Value};
            use crate::jsonrpc::{OdooApiMethod};
            use super::{#ident_fn, #ident_struct, #ident_response};

            #[cfg(feature = "async")]
            #[doc=#doc]
            #[allow(clippy::too_many_arguments)]
            pub async fn #ident_fn_async(url: &str, #(#fields_args),*) -> crate::jsonrpc::Result<super::#ident_response> {
                let request = super::#ident_fn(#(#fields_call),*)?;
                let client = ::reqwest::Client::new();
                let response: crate::jsonrpc::OdooApiResponse<#ident_struct> = client.post(url)
                    .header("X-Odoo-Dbfilter", db.clone())
                    .json(&request)
                    .send().await?
                    .json().await?;

                match response {
                    crate::jsonrpc::OdooApiResponse::Success(resp) => {
                        Ok(resp.result)
                    },
                    crate::jsonrpc::OdooApiResponse::Error(resp) => {
                        if &resp.error.message == "Odoo Server Error" {
                            Err(crate::jsonrpc::Error::OdooServerError(resp.error))
                        }
                        else if &resp.error.message == "404: Not Found" {
                            Err(crate::jsonrpc::Error::OdooNotFoundError(resp.error))
                        }
                        else if &resp.error.message == "Odoo Session Expired" {
                            Err(crate::jsonrpc::Error::OdooSessionExpiredError(resp.error))
                        }
                        else {
                            Err(crate::jsonrpc::Error::OdooError(resp.error))
                        }
                    }
                }
            }
        }
    ))
}

fn generate_call_blocking(
    ident_struct: &Ident,
    ident_response: &Ident,
    ident_fn: &Ident,
    fields_args: &Vec<TokenStream>,
    fields_call: &Vec<TokenStream>,
    doc_base: &str,
    args: &OdooApiRequestArgs,
) -> Result<TokenStream> {
    let name_fn_blocking = format!("{}_blocking", &ident_fn.to_string());
    let ident_fn_blocking = Ident::new(&name_fn_blocking, Span::call_site());
    let doc = format!("\
        {}\
        See the [`types::{}::{}`](crate::jsonrpc::types::{}::{}) function for \
        more information on this API method call, or the [`blocking`](crate::jsonrpc::blocking) \
        module for more information on making blocking requests.",
        doc_base,
        &args.service, &args.method,
        &args.service, &args.method,
    );

    Ok(quote!(
        pub(crate) mod #ident_fn_blocking {
            use serde_json::{Value};
            use crate::jsonrpc::{OdooApiMethod};
            use super::{#ident_fn, #ident_struct, #ident_response};

            #[cfg(feature = "blocking")]
            #[doc=#doc]
            #[allow(clippy::too_many_arguments)]
            pub fn #ident_fn_blocking(url: &str, #(#fields_args),*) -> crate::jsonrpc::Result<super::#ident_response> {
                let request = super::#ident_fn(#(#fields_call),*)?;
                let client = ::reqwest::blocking::Client::new();
                let response: crate::jsonrpc::OdooApiResponse<#ident_struct> = client.post(url)
                    .header("X-Odoo-Dbfilter", db.clone())
                    .json(&request)
                    .send()?
                    .json()?;

                match response {
                    crate::jsonrpc::OdooApiResponse::Success(resp) => {
                        Ok(resp.result)
                    },
                    crate::jsonrpc::OdooApiResponse::Error(resp) => {
                        if &resp.error.message == "Odoo Server Error" {
                            Err(crate::jsonrpc::Error::OdooServerError(resp.error))
                        }
                        else if &resp.error.message == "404: Not Found" {
                            Err(crate::jsonrpc::Error::OdooNotFoundError(resp.error))
                        }
                        else if &resp.error.message == "404: Not Found" {
                            Err(crate::jsonrpc::Error::OdooSessionExpiredError(resp.error))
                        }
                        else {
                            Err(crate::jsonrpc::Error::OdooError(resp.error))
                        }
                    }
                }
            }
        }
    ))
}

fn generate_api_method_impl(
    ident_struct: &Ident,
    ident_response: &Ident,
    args: &OdooApiRequestArgs,
) -> Result<TokenStream> {
    let service = &args.service;
    let method = &args.method;
    Ok(quote! {
        impl super::OdooApiMethod for #ident_struct {
            type Response = #ident_response;

            fn describe_odoo_api_method(&self) -> (&'static str, &'static str) {
                (#service, #method)
            }

            fn parse_json_response(&self, json_data: &str) -> ::serde_json::Result<super::OdooApiResponse<Self>> {
                ::serde_json::from_str(json_data)
            }
        }
    })
}

fn generate_serialize_impl(ident_struct: &Ident, fields: &[Field]) -> Result<TokenStream> {
    let field_count = fields.iter().map(|field| &field.ident).len();
    let field_names = fields.iter().map(|field| &field.ident);

    Ok(quote! {
        impl ::serde::Serialize for #ident_struct {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer,
            {
                use ::serde::ser::SerializeSeq;

                let mut state = serializer.serialize_seq(Some(#field_count))?;
                #(
                    state.serialize_element(&self.#field_names)?;
                )*
                state.end()
            }
        }
    })
}

fn generate_try_from_impl(ident_response: &Ident) -> Result<TokenStream> {
    Ok(quote!(
        impl TryFrom<String> for #ident_response {
            type Error = crate::jsonrpc::Error;

            fn try_from(value: String) -> ::std::result::Result<#ident_response, crate::jsonrpc::Error> {
                Ok(serde_json::from_str(&value)?)
            }
        }

        impl TryFrom<serde_json::Value> for #ident_response {
            type Error = crate::jsonrpc::Error;

            fn try_from(value: serde_json::Value) -> ::std::result::Result<#ident_response, crate::jsonrpc::Error> {
                Ok(serde_json::from_value(value)?)
            }
        }
    ))
}
