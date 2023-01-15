use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote, ToTokens};
use syn::{FieldsNamed, Ident, Type};

use crate::common::{ItemStructNamed, MacroArguments};
use crate::{Error, Result};

struct OdooApiArgs {
    /// The JSON-RPC "service"
    service: String,

    /// The JSON-RPC "method"
    method: String,

    /// Is authentication required, optional, should we skip generating the
    /// OdooClient impl?
    auth: Option<bool>,

    /// Optionally specify a name for the OdooClient impl
    name: Option<String>,
}

impl TryFrom<MacroArguments> for OdooApiArgs {
    type Error = Error;

    fn try_from(value: MacroArguments) -> Result<Self> {
        let mut service = None;
        let mut method = None;
        let mut auth = None;
        let mut name = None;

        for arg in value.into_iter() {
            match (arg.key.as_str(), arg.value, arg.span) {
                ("service", val, span) => {
                    service = Some(val.try_into().map_err(|_| (
                        "invalid value, expected String (e.g., `service = \"object\"`)",
                        Some(span)
                    ))?);
                },
                ("method", val, span) => {
                    method = Some(val.try_into().map_err(|_| (
                        "invalid value, expected String (e.g., `method = \"execute_kw\"`)",
                        Some(span)
                    ))?);
                },
                ("auth", val, span) => {
                    auth = Some(val.try_into().map_err(|_| (
                        "invalid value, expected String (e.g., `auth = false`)",
                        Some(span)
                    ))?);
                },
                ("name", val, span) => {
                    name = Some(val.try_into().map_err(|_| (
                        "invalid value, expected String (e.g., `name = \"my_execute_kw\"`)",
                        Some(span)
                    ))?);
                },

                (key, _val, span) => Err((
                    format!(
                        "Invalid argument `{}`. Valid arguments are: service, method, auth, name",
                        key
                    ),
                    Some(span),
                ))?
            }
        }

        Ok(Self {
            service: service
                .ok_or("The \"service\" key is required (e.g., `service = \"object\"`)")?,
            method: method
                .ok_or("The \"method\" key is required (e.g., `method = \"execute\"`)")?,
            auth,
            name,
        })
    }
}

pub(crate) fn odoo_api(args: MacroArguments, input: ItemStructNamed) -> Result<TokenStream2> {
    let args: OdooApiArgs = args.try_into()?;

    // fetch the struct name (and some variations)
    let name_struct = input.item.ident.to_string();
    let name_response = format!("{}Response", &name_struct);
    let name_call = if let Some(name) = &args.name {
        name.clone()
    } else {
        args.method.clone()
    };
    let ident_struct = input.item.ident.clone();
    let ident_response = Ident::new(&name_response, Span::call_site());
    let ident_call = Ident::new(&name_call, Span::call_site());

    // build a quick doc-comment directing users from the function impl,
    // back to the struct (where we have examples/etc)
    let doc_call = format!(
        "{}\n\nSee [`{}`](crate::service::{}::{}) for more info.",
        &input.doc_head, &name_struct, &args.service, &name_struct
    );

    // build the TokenStreams
    let out_params = impl_params(&ident_struct, &ident_response)?;
    let out_method = impl_method(&ident_struct, &args)?;
    let out_client = impl_client(&ident_struct, &ident_call, &args, &input.fields, &doc_call)?;

    // output the result!
    Ok(quote!(
        #input
        #out_params
        #out_method
        #out_client
    ))
}

/// Output the [`JsonRpcParams`](odoo_api::jsonrpc::JsonRpcParams) impl
fn impl_params(ident_struct: &Ident, ident_response: &Ident) -> Result<TokenStream2> {
    Ok(quote! {
        impl odoo_api::jsonrpc::JsonRpcParams for #ident_struct {
            type Container<T> = odoo_api::jsonrpc::OdooApiContainer <Self>;
            type Response = #ident_response;

            fn build(self, id: odoo_api::jsonrpc::JsonRpcId) -> odoo_api::jsonrpc::JsonRpcRequest<Self> { self._build(id) }
        }
    })
}

/// Output the OdooApiMethod impl
fn impl_method(ident_struct: &Ident, args: &OdooApiArgs) -> Result<TokenStream2> {
    let service = &args.service;
    let method = &args.method;
    Ok(quote! {
        impl odoo_api::jsonrpc::OdooApiMethod for #ident_struct {
            fn describe(&self) -> (&'static str, &'static str) {
                (#service, #method)
            }
            fn endpoint(&self) -> &'static str {
                "/jsonrpc"
            }
        }
    })
}

/// Output the OdooClient impl
fn impl_client(
    ident_struct: &Ident,
    ident_call: &Ident,
    args: &OdooApiArgs,
    fields: &FieldsNamed,
    doc: &str,
) -> Result<TokenStream2> {
    if args.auth.is_none() {
        // The `auth` key wasn't passed, so we'll just skip the OdooClient impl
        return Ok(quote!());
    }

    let auth = args.auth.unwrap();

    // parse the `auth` argument options
    let (auth_generic, auth_type) = if auth {
        // no generic, we're implementing for the concrete `Authed` type
        (quote!(), quote!(odoo_api::client::Authed))
    } else {
        // auth not required, so we'll implement for any `impl AuthState`
        (quote!(S: odoo_api::client::AuthState), quote!(S))
    };

    // parse fields
    let mut field_assigns = Vec::new();
    let mut field_arguments = Vec::new();
    for field in fields.named.clone() {
        let ident = field.ident.unwrap();
        let ty = if let Type::Path(path) = field.ty {
            path
        } else {
            continue;
        };
        let name = ident.to_string();
        let path = ty.clone().into_token_stream().to_string();
        match (name.as_str(), path.as_str(), auth) {
            // special cases (data fetched from the `client.auth` struct)
            ("database", "String", true) => {
                field_assigns.push(quote!(database: self.auth.database.clone()));
            }
            ("db", "String", true) => {
                field_assigns.push(quote!(db: self.auth.database.clone()));
            }
            ("uid", "OdooId", true) => {
                field_assigns.push(quote!(uid: self.auth.uid));
            }
            ("login", "String", true) => {
                field_assigns.push(quote!(login: self.auth.login.clone()));
            }
            ("password", "String", true) => {
                field_assigns.push(quote!(password: self.auth.password.clone()));
            }

            // strings are passed by ref
            //TODO: Into<String> more suitable?
            (_, "String", _) => {
                field_assigns.push(quote!(#ident: #ident.into()));
                field_arguments.push(quote!(#ident: &str));
            }

            // all other fields are passed as-is
            (_, _, _) => {
                field_assigns.push(quote!(#ident: #ident));
                field_arguments.push(quote!(#ident: #ty));
            }
        }
    }

    Ok(quote! {
        #[cfg(not(feature = "types-only"))]
        impl<I: odoo_api::client::RequestImpl, #auth_generic> odoo_api::client::OdooClient<#auth_type, I> {
            #[doc=#doc]
            pub fn #ident_call(&mut self, #(#field_arguments),*) -> odoo_api::client::OdooRequest< #ident_struct , I> {
                let #ident_call = #ident_struct {
                    #(#field_assigns),*
                };

                let endpoint = self.build_endpoint(#ident_call.endpoint());
                self.build_request(
                    #ident_call,
                    &endpoint
                )
            }
        }
    })
}
