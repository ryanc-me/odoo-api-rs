//! Helper macros for the [`odoo_api`] crate

extern crate proc_macro;
use self::proc_macro::{TokenStream};
use quote::{quote, quote_spanned, ToTokens };
use syn::{Token, parse, Ident, Lit, LitStr, parse_macro_input, Meta, MetaNameValue, ItemStruct, Type};
use syn::parse::{Parse, ParseStream};
use proc_macro2::{self, Span};
use convert_case::{Case, Casing};

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

#[proc_macro_attribute]
pub fn odoo_api_request(args: TokenStream, input: TokenStream) -> TokenStream {
    let macro_args = parse_macro_input!(args as OdooApiRequestArgs);

    // make sure the macro was applied to a struct, then parse the stream into
    // an ItemStruct token
    let input: ItemStruct = match parse(input) {
        Ok(input) => { input },
        Err(_err) => {
            let span = Span::call_site();
            return TokenStream::from(quote_spanned! {span=>
                compile_error!("expected a struct");
            });
        }
    };

    // make sure the struct has named fields, then parse
    let fields = match &input.fields {
        syn::Fields::Named(fields) => { &fields.named },
        _ => {
            let span = Span::call_site();
            return TokenStream::from(quote_spanned! {span=>
                compile_error!("expected a struct with named fields")
            });
        }
    };

    // make sure the struct doesn't have any generic params
    if !input.generics.params.is_empty() {
        let span = input.ident.span();
        return TokenStream::from(quote_spanned! {span=>
            compile_error!("a struct tagged with `odoo_api_request` cannot have generic params");
        });
    }

    // fetch the struct name
    let struct_name = &input.ident;

    // parse the field names & types
    let mut args = Vec::<proc_macro2::TokenStream>::new();
    let mut assigns = Vec::<proc_macro2::TokenStream>::new();
    let mut generics = Vec::<proc_macro2::TokenStream>::new();
    let mut generic_n = 0;
    for field in fields.clone() {
        match field.ty {
            Type::Path(path) if path.clone().into_token_stream().to_string() == "String" => {
                generic_n += 1;
                let ident = field.ident;
                let ty = Ident::new(&format!("STR{}", generic_n), Span::call_site());

                args.push(
                    quote!(#ident: #ty)
                );
                assigns.push(
                    quote!(#ident: #ident.into())
                );
                generics.push(
                    quote!(#ty: Into<String>)
                )
            },
            _ => {
                let ident = field.ident;
                let ty = field.ty;
                args.push(
                    quote!(#ident: #ty)
                );
                assigns.push(
                    quote!(#ident: #ident)
                );
            }
        }
    }

    // extract doc comments from the original struct
    let doc_attrs: String = input.attrs.iter()
        .map(|attr| {
            if !attr.path.is_ident("doc") {
                return None
            }

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

    // build new doc comments
    let mut doc: String;
    if !doc_attrs.is_empty() {
        doc = doc_attrs;
    }
    else {
        doc = format!(" For details, please see the [`{}`] struct.", struct_name.to_string());
    }
    if !generics.is_empty() {
        doc.push_str("\n\nNOTE: The `Into<String>` type parameters allow you to pass either `&str` or `String`.");
    }

    // the `new()` impl
    let out_fn_new = quote! {
        impl #struct_name {
            #[doc=#doc]
            pub fn new<#(#generics),*>(#(#args),*) -> Self {
                Self {
                    #(
                        #assigns,
                    )*
                }
            }
        }
    };

    // the 'call' impl (e.g. `execute()` or `execute_kw()`)
    let fn_name = Ident::new(
        &struct_name.to_string().to_case(Case::Snake),
        Span::call_site()
    );
    let out_fn_call = quote! {
        #[cfg(feature = "rand")]
        #[doc=#doc]
        pub fn #fn_name<#(#generics),*>(#(#args),*) -> crate::OdooApiRequest<#struct_name> {
            use rand::{Rng, thread_rng};
            rng = thread_rng();
            crate::OdooApiRequest {
                version: crate::JsonRpcVersion::V2,
                method: crate::JsonRpcMethod::Call,
                id: rng.gen_range(1..10000),
                params: crate::JsonRpcRequestParams {
                    args: #struct_name {
                        #(
                            #assigns,
                        )*
                    }
                }
            }
        }

        #[cfg(not(feature = "rand"))]
        #[doc=#doc]
        pub fn #fn_name<#(#generics),*>(#(#args),*) -> crate::OdooApiRequest<#struct_name> {
            crate::OdooApiRequest {
                version: crate::JsonRpcVersion::V2,
                method: crate::JsonRpcMethod::Call,
                id: 10000,
                params: crate::JsonRpcRequestParams {
                    args: #struct_name {
                        #(
                            #assigns,
                        )*
                    }
                }
            }
        }
    };

    // the flattening Serialize impl
    let field_count = fields.iter().map(|field| &field.ident).len();
    let field_names = fields.iter().map(|field| &field.ident);
    let out_serialize_impl = quote! {
        impl ::serde::Serialize for #struct_name {
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
    };

    // the OdooApiMethod impl
    let service = macro_args.service;
    let method = macro_args.method;
    let response_struct = Ident::new(
        &format!("{}Response", &struct_name.to_string()),
        Span::call_site()
    );
    let out_method_impl = quote! {
        impl crate::OdooApiMethod for #struct_name {
            type Response = #response_struct;

            fn describe_odoo_api_method(&self) -> (&'static str, &'static str) {
                (#service, #method)
            }

            fn parse_json_response(&self, json_data: &str) -> ::serde_json::Result<crate::OdooApiResponse<Self>> {
                ::serde_json::from_str(json_data)
            }
        }
    };

    TokenStream::from(quote! {
        #input
        #out_fn_new
        #out_fn_call
        #out_serialize_impl
        #out_method_impl
    })
}
