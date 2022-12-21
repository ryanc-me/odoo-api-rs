//! Helper macros for the `odoo_api` crate

use proc_macro;
use quote::{quote, quote_spanned, ToTokens };
use syn::{Token, Ident, Lit, LitStr, Meta, MetaNameValue, ItemStruct, Type, Field};
use syn::parse::{Parse, ParseStream};
use proc_macro2::{Span, TokenStream};
use convert_case::{Case, Casing};

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
}

impl Parse for OdooApiRequestArgs {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let service: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let method: LitStr = input.parse()?;

        Ok(OdooApiRequestArgs {
            service: service.value(),
            method: method.value(),
        })
    }
}

fn odoo_api_request_impl(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> Result<TokenStream> {
    // parse args and input (ensuring the macro was applied to a struct)
    let args: OdooApiRequestArgs = syn::parse(args).map_err(|e| e.to_compile_error())?;
    let input: ItemStruct = syn::parse(input).map_err(|e| e.to_compile_error())?;

    // make sure the struct has named fields, then parse
    let fields: Vec<Field> = match &input.fields {
        syn::Fields::Named(fields) => { fields.named.clone().into_iter().collect() },
        _ => {
            let span = Span::call_site();
            return Err(quote_spanned! {span=>
                compile_error!("expected a struct with named fields")
            }.into());
        }
    };

    // make sure the struct doesn't have any generic params
    if !input.generics.params.is_empty() {
        let span = input.ident.span();
        return Err(quote_spanned!{span=>
            compile_error!("a struct tagged with `odoo_api_request` cannot have generic params");
        }.into());
    }

    // fetch the struct name (and some variations)
    let name_struct = input.ident.clone().to_string();
    let name_response = format!("{}Response", &name_struct);
    let name_fn = name_struct.to_case(Case::Snake);
    let ident_struct = &input.ident;
    let ident_response = Ident::new(&name_response, Span::call_site());
    let ident_fn = Ident::new(&name_fn, Span::call_site());

    // extract doc comments from the original struct
    let mut doc = input.attrs.iter()
        .map(|attr| {
            if !attr.path.is_ident("doc") { return None }
            match attr.parse_meta() {
                Ok(meta) => {
                    match meta {
                        Meta::NameValue(MetaNameValue { lit: Lit::Str(lit_str), ..}) => {
                            Some(lit_str.value())
                        },
                        _ => { None }
                    }
                },
                _ => { None }
            }
        })
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");
    if doc.is_empty() {
        doc.push_str(&format!(" For details, please see the [`{}`] struct.", &name_struct));
    }

    // parse the field names & types
    let mut has_db = false;
    let mut fields_args = Vec::<TokenStream>::new();
    let mut fields_assigns = Vec::<TokenStream>::new();
    let mut fields_call = Vec::<TokenStream>::new();
    for field in &fields {
        if let Type::Path(path) = &field.ty {
            if let Some(ident) = &field.ident {
                if ident.to_string() == "db" { has_db = true }
                let type_string = path.clone().into_token_stream().to_string();
                if type_string == "String" {
                    // rather than requiring full Strings, we'll accept &str and convert
                    fields_args.push(quote!(#ident: &str));
                    fields_assigns.push(quote!(#ident: #ident.to_string()));
                    fields_call.push(quote!(#ident));
                }
                else {
                    fields_args.push(quote!(#ident: #path));
                    fields_assigns.push(quote!(#ident: #ident));
                    fields_call.push(quote!(#ident));
                }
            }
        }
    }

    // finally, generate the TokenStreams
    let out_call = generate_call(&ident_struct, &ident_fn, &fields_args, &fields_assigns, &doc)?;
    let out_call_async = generate_call_async(&ident_struct, &ident_response, &ident_fn, &fields_args, &fields_call, &doc, has_db)?;
    let out_call_blocking = generate_call_blocking(&ident_struct, &ident_response, &ident_fn, &fields_args, &fields_call, &doc, has_db)?;
    let out_api_method_impl = generate_api_method_impl(&ident_struct, &ident_response, &args)?;
    let out_serialize_impl = generate_serialize_impl(&ident_struct, &fields)?;

    Ok(quote!(
        #input
        #out_call
        #out_call_async
        #out_call_blocking
        #out_api_method_impl
        #out_serialize_impl
    ))
}

fn generate_call(ident_struct: &Ident, ident_fn: &Ident, fields_args: &Vec<TokenStream>, fields_assigns: &Vec<TokenStream>, doc: &str) -> Result<TokenStream> {
    Ok(quote! {
        #[doc=#doc]
        pub fn #ident_fn(#(#fields_args),*) -> super::OdooApiRequest<#ident_struct> {
            use ::rand::{Rng, thread_rng};
            let mut rng = thread_rng();
            super::OdooApiRequest {
                version: super::JsonRpcVersion::V2,
                method: super::JsonRpcMethod::Call,
                id: rng.gen_range(1..10000),
                params: super::JsonRpcRequestParams {
                    args: #ident_struct {
                        #(
                            #fields_assigns,
                        )*
                    }
                }
            }
        }
    })
}

fn generate_call_async(ident_struct: &Ident, ident_response: &Ident, ident_fn: &Ident, fields_args: &Vec<TokenStream>, fields_call: &Vec<TokenStream>, doc: &str, has_db: bool) -> Result<TokenStream> {
    let name_fn_async = format!("{}_async", &ident_fn.to_string());
    let ident_fn_async = Ident::new(&name_fn_async, Span::call_site());

    let db_header = match has_db {
        true => { quote!(.header("X-Odoo-Dbfilter", db.clone())) },
        false => { quote!() },
    };

    Ok(quote!(
        #[cfg(feature = "nonblocking")]
        #[doc=#doc]
        pub async fn #ident_fn_async(url: &str, #(#fields_args),*) -> super::Result<#ident_response> {
            let request = self::#ident_fn(#(#fields_call),*);
            let client = ::reqwest::Client::new();
            let response: super::OdooApiResponse<#ident_struct> = client.post(url)
                #db_header
                .json(&request)
                .send().await?
                .json().await?;

            match response {
                super::OdooApiResponse::Success(resp) => {
                    Ok(resp.result)
                },
                super::OdooApiResponse::Error(resp) => {
                    if &resp.error.message == "Odoo Server Error" {
                        Err(super::Error::OdooServerError(resp.error))
                    }
                    else if &resp.error.message == "404: Not Found" {
                        Err(super::Error::OdooNotFoundError(resp.error))
                    }
                    else if &resp.error.message == "404: Not Found" {
                        Err(super::Error::OdooSessionExpiredError(resp.error))
                    }
                    else {
                        Err(super::Error::OdooError(resp.error))
                    }
                }
            }
        }
    ))
}

fn generate_call_blocking(ident_struct: &Ident, ident_response: &Ident, ident_fn: &Ident, fields_args: &Vec<TokenStream>, fields_call: &Vec<TokenStream>, doc: &str, has_db: bool) -> Result<TokenStream> {
    let name_fn_blocking = format!("{}_blocking", &ident_fn.to_string());
    let ident_fn_blocking = Ident::new(&name_fn_blocking, Span::call_site());

    let db_header;
    if has_db {
        db_header = quote!(.header("X-Odoo-Dbfilter", db.clone()));
    }
    else {
        db_header = quote!();
    }

    Ok(quote!(
        #[cfg(feature = "blocking")]
        #[doc=#doc]
        pub fn #ident_fn_blocking(url: &str, #(#fields_args),*) -> super::Result<#ident_response> {
            let request = self::#ident_fn(#(#fields_call),*);
            let client = ::reqwest::blocking::Client::new();
            let response: super::OdooApiResponse<#ident_struct> = client.post(url)#db_header
                .json(&request)
                .send()?
                .json()?;

            match response {
                super::OdooApiResponse::Success(resp) => {
                    Ok(resp.result)
                },
                super::OdooApiResponse::Error(resp) => {
                    if &resp.error.message == "Odoo Server Error" {
                        Err(super::Error::OdooServerError(resp.error))
                    }
                    else if &resp.error.message == "404: Not Found" {
                        Err(super::Error::OdooNotFoundError(resp.error))
                    }
                    else if &resp.error.message == "404: Not Found" {
                        Err(super::Error::OdooSessionExpiredError(resp.error))
                    }
                    else {
                        Err(super::Error::OdooError(resp.error))
                    }
                }
            }
        }
    ))
}

fn generate_api_method_impl(ident_struct: &Ident, ident_response: &Ident, args: &OdooApiRequestArgs) -> Result<TokenStream> {
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

fn generate_serialize_impl(ident_struct: &Ident, fields: &Vec<Field>) -> Result<TokenStream> {
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

#[proc_macro_attribute]
pub fn odoo_api_request(args: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match odoo_api_request_impl(args, input) {
        Ok(ts) => { ts },
        Err(err) => {
            match err {
                Error::TokenStream(ts) => { ts },
                Error::MacroError(s) => { quote!(compile_error!(#s)) }
            }
        }
    }.into()
}

// fn test() {
//     {
//         let macro_args = parse_macro_input!(args as OdooApiRequestArgs);

//         // make sure the macro was applied to a struct, then parse the stream into
//         // an ItemStruct token
//         let input: ItemStruct = match parse(input) {
//             Ok(input) => { input },
//             Err(_err) => {
//                 let span = Span::call_site();
//                 return TokenStream::from(quote_spanned! {span=>
//                     compile_error!("expected a struct");
//                 });
//             }
//         };

//         // make sure the struct has named fields, then parse
//         let fields = match &input.fields {
//             syn::Fields::Named(fields) => { &fields.named },
//             _ => {
//                 let span = Span::call_site();
//                 return TokenStream::from(quote_spanned! {span=>
//                     compile_error!("expected a struct with named fields")
//                 });
//             }
//         };

//         // make sure the struct doesn't have any generic params
//         if !input.generics.params.is_empty() {
//             let span = input.ident.span();
//             return TokenStream::from(quote_spanned! {span=>
//                 compile_error!("a struct tagged with `odoo_api_request` cannot have generic params");
//             });
//         }

//         // fetch the struct name
//         let struct_name = &input.ident;

//         // parse the field names & types
//         let mut found_db = false;
//         let mut args = Vec::<TokenStream>::new();
//         let mut assigns = Vec::<TokenStream>::new();
//         let generics = Vec::<TokenStream>::new();
//         let mut fn_args = Vec::<TokenStream>::new();
//         let mut generic_n = 0;
//         for field in fields.clone() {
//             match field.ty {
//                 Type::Path(path) if path.clone().into_token_stream().to_string() == "String" => {
//                     generic_n += 1;
//                     let ident = field.ident.unwrap();
//                     let ident_str = ident.clone().to_string();
//                     if ident_str == "db" { found_db = true }
//                     let _ty = Ident::new(&format!("STR{}", generic_n), Span::call_site());

//                     fn_args.push(
//                         quote!(#ident)
//                     );
//                     args.push(
//                         quote!(#ident: &str)
//                     );
//                     assigns.push(
//                         quote!(#ident: #ident.to_string())
//                     );
//                     // generics.push(
//                     //     quote!(#ty: Into<String>)
//                     // )
//                 },
//                 _ => {
//                     let ident = field.ident;
//                     let ty = field.ty;
//                     fn_args.push(
//                         quote!(#ident)
//                     );
//                     args.push(
//                         quote!(#ident: #ty)
//                     );
//                     assigns.push(
//                         quote!(#ident: #ident)
//                     );
//                 }
//             }
//         }

//         // extract doc comments from the original struct
//         let doc_attrs: String = input.attrs.iter()
//             .map(|attr| {
//                 if !attr.path.is_ident("doc") {
//                     return None
//                 }

//                 match attr.parse_meta() {
//                     Ok(meta) => {
//                         match meta {
//                             Meta::NameValue(MetaNameValue { lit: Lit::Str(lit_str), ..}) => {
//                                 Some(lit_str.value())
//                             },
//                             _ => { None }
//                         }
//                     },
//                     _ => { None }
//                 }
//             })
//             .flatten()
//             .collect::<Vec<String>>()
//             .join("\n");

//         // build new doc comments
//         let mut doc: String;
//         if !doc_attrs.is_empty() {
//             doc = doc_attrs;
//         }
//         else {
//             doc = format!(" For details, please see the [`{}`] struct.", struct_name.to_string());
//         }
//         if !generics.is_empty() {
//             doc.push_str("\n\nNOTE: The `Into<String>` type parameters allow you to pass either `&str` or `String`.");
//         }
//     }

//     // the 'call' impl (e.g. `execute()` or `execute_kw()`)
//     let fn_name = struct_name.to_string().to_case(Case::Snake);
//     let fn_name_ident  = Ident::new(
//         &fn_name,
//         Span::call_site()
//     );
//     let out_fn_call = quote! {
//         #[doc=#doc]
//         pub fn #fn_name_ident<#(#generics),*>(#(#args),*) -> super::OdooApiRequest<#struct_name> {
//             use ::rand::{Rng, thread_rng};
//             let mut rng = thread_rng();
//             super::OdooApiRequest {
//                 version: super::JsonRpcVersion::V2,
//                 method: super::JsonRpcMethod::Call,
//                 id: rng.gen_range(1..10000),
//                 params: super::JsonRpcRequestParams {
//                     args: #struct_name {
//                         #(
//                             #assigns,
//                         )*
//                     }
//                 }
//             }
//         }
//     };

//     // the flattening Serialize impl
//     let field_count = fields.iter().map(|field| &field.ident).len();
//     let field_names = fields.iter().map(|field| &field.ident);
//     let out_serialize_impl = quote! {
//         impl ::serde::Serialize for #struct_name {
//             fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//             where
//                 S: ::serde::Serializer,
//             {
//                 use ::serde::ser::SerializeSeq;

//                 let mut state = serializer.serialize_seq(Some(#field_count))?;
//                 #(
//                     state.serialize_element(&self.#field_names)?;
//                 )*
//                 state.end()
//             }
//         }
//     };

//     // the OdooApiMethod impl
//     let service = macro_args.service;
//     let method = macro_args.method;
//     let response_struct = Ident::new(
//         &format!("{}Response", &struct_name.to_string()),
//         Span::call_site()
//     );
//     let out_method_impl = quote! {
//         impl super::OdooApiMethod for #struct_name {
//             type Response = #response_struct;

//             fn describe_odoo_api_method(&self) -> (&'static str, &'static str) {
//                 (#service, #method)
//             }

//             fn parse_json_response(&self, json_data: &str) -> ::serde_json::Result<super::OdooApiResponse<Self>> {
//                 ::serde_json::from_str(json_data)
//             }
//         }
//     };

//     // the reqwest async impl
//     let fn_name_async = format!("{}_async", fn_name);
//     let fn_name_async_ident = Ident::new(
//         &fn_name_async,
//         Span::call_site()
//     );
//     let mut db_arg: Option<_> = None;
//     if !found_db && true{
//         db_arg = Some(quote!(db: &str, ));
//     }
//     let out_reqwest_async = quote!(
//         #[doc=#doc]
//         pub async fn #fn_name_async_ident<#(#generics),*>(url: &str, #db_arg #(#args),*) -> super::Result<#response_struct> {
//             let request = self::#fn_name_ident(
//                 #(#fn_args),*
//             );
//             let client = ::reqwest::Client::new();
//             let response: super::OdooApiResponse<#struct_name> = client.post(url)
//                 .header("X-Odoo-Dbfilter", db.clone())
//                 .json(&request)
//                 .send().await?
//                 .json().await?;

//             match response {
//                 super::OdooApiResponse::Success(resp) => {
//                     Ok(resp.result)
//                 },
//                 super::OdooApiResponse::Error(resp) => {
//                     if &resp.error.message == "Odoo Server Error" {
//                         Err(super::Error::OdooServerError(resp.error))
//                     }
//                     else if &resp.error.message == "404: Not Found" {
//                         Err(super::Error::OdooNotFoundError(resp.error))
//                     }
//                     else if &resp.error.message == "404: Not Found" {
//                         Err(super::Error::OdooSessionExpiredError(resp.error))
//                     }
//                     else {
//                         Err(super::Error::OdooError(resp.error))
//                     }
//                 }
//             }
//         }
//     );

//     // the reqwest blocking impl
//     let fn_name_blocking = format!("{}_blocking", fn_name);
//     let fn_name_blocking_ident = Ident::new(
//         &fn_name_blocking,
//         Span::call_site()
//     );
//     let out_reqwest_blocking = quote!(

//         #[cfg(feature = "blocking")]
//         #[doc=#doc]
//         pub fn #fn_name_blocking_ident<#(#generics),*>(#(#args),*) -> super::Result<()> {

//             Ok(())
//         }
//     );

//     TokenStream::from(quote! {
//         #input
//         #out_fn_call
//         #out_serialize_impl
//         #out_method_impl
//         #out_reqwest_async
//         #out_reqwest_blocking
//     })
// }
