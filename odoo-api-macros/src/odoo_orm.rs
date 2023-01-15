use proc_macro2::{TokenStream as TokenStream2, Span};
use quote::{quote, ToTokens};
use syn::{FieldsNamed, Ident, Type};

use crate::common::{ItemStructNamed, MacroArguments};
use crate::{Error, Result};

#[derive(Debug)]
struct OdooOrmArgs {
    /// The ORM "method" (e.g. 'write', 'read_group', etc)
    method: String,

    /// Optionally specify a name for the OdooClient impl
    name: Option<String>,

    /// A list of the positional arguments
    args: Vec<String>,

    /// A list of the keyword arguments
    kwargs: Vec<String>,
}

impl TryFrom<MacroArguments> for OdooOrmArgs {
    type Error = Error;

    fn try_from(value: MacroArguments) -> Result<Self> {
        // let from_args = parse_args(value)?;

        let mut method = None;
        let mut name = None;
        let mut args = None;
        let mut kwargs = None;

        for arg in value.into_iter() {
            match (arg.key.as_str(), arg.value, arg.span) {
                ("method", val, span) => {
                    method = Some(val.try_into().map_err(|_| (
                        "invalid value, expected String (e.g., `method = \"read\"`)",
                        Some(span)
                    ))?);
                },
                ("name", val, span) => {
                    name = Some(val.try_into().map_err(|_| (
                        "invalid value, expected String (e.g., `name = \"my_custom_read\"`)",
                        Some(span)
                    ))?);
                },
                ("args", val, span) => {
                    args = Some(val.try_into().map_err(|_| (
                        "invalid value, expected String (e.g., `args = [\"list\", \"of\", \"literals\"]`)",
                        Some(span)
                    ))?);
                },
                ("kwargs", val, span) => {
                    kwargs = Some(val.try_into().map_err(|_| (
                        "invalid value, expected String (e.g., `kwargs = [\"list\", \"of\", \"literals\"]`)",
                        Some(span)
                    ))?);
                },

                (key, _val, span) => Err((
                    format!(
                        "Invalid argument `{}`. Valid arguments are: method, name, args, kwargs",
                        key
                    ),
                    Some(span),
                ))?
            }
        }

        Ok(Self {
            method: method.ok_or(
                "The \"method\" key is required (e.g., `method = \"read_group\"`)",
            )?,
            name,
            args: args.ok_or(
                "The \"args\" key is required, even if you only pass an empty array (e.g., `args = []`)"
            )?,
            kwargs: kwargs.ok_or(
                "The \"kwargs\" key is required, even if you only pass an empty array (e.g., `kwargs = []`)"
            )?,
        })
    }
}

pub(crate) fn odoo_orm(args: MacroArguments, input: ItemStructNamed) -> Result<TokenStream2> {
    let args: OdooOrmArgs = args.try_into()?;

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
        "{}\n\nSee [`{}`](crate::service::orm::{}) for more info.",
        &input.doc_head, &name_struct, &name_struct
    );

    // build the TokenStreams
    let out_params = impl_params(&ident_struct, &ident_response)?;
    let out_method = impl_method(&ident_struct)?;
    let out_client = impl_client(&ident_struct, &ident_call, &input.fields, &doc_call)?;
    let out_serialize = impl_serialize(&ident_struct, &args)?;

    // output the result!
    Ok(quote!(
        #input
        #out_params
        #out_method
        #out_client
        #out_serialize
    ))
}

/// Output the [`JsonRpcParams`](odoo_api::jsonrpc::JsonRpcParams) impl
pub(crate) fn impl_params(ident_struct: &Ident, ident_response: &Ident) -> Result<TokenStream2> {
    Ok(quote! {
        impl odoo_api::jsonrpc::JsonRpcParams for #ident_struct {
            type Container<T> = odoo_api::jsonrpc::OdooOrmContainer <Self>;
            type Response = #ident_response;

            fn build(self, id: odoo_api::jsonrpc::JsonRpcId) -> odoo_api::jsonrpc::JsonRpcRequest<Self> { self._build(id) }
        }
    })
}

/// Output the OdooApiMethod impl
fn impl_method(ident_struct: &Ident) -> Result<TokenStream2> {
    Ok(quote! {
        impl odoo_api::jsonrpc::OdooOrmMethod for #ident_struct {
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
    fields: &FieldsNamed,
    doc: &str,
) -> Result<TokenStream2> {
    // parse the `auth` argument options
    let auth_generic = quote!();
    let auth_type = quote!(odoo_api::client::Authed);

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
        match (name.as_str(), path.as_str()) {
            // special cases (data fetched from the `client.auth` struct)
            ("database", "String") => {
                field_assigns.push(quote!(database: self.auth.database.clone()));
            }
            ("db", "String") => {
                field_assigns.push(quote!(db: self.auth.database.clone()));
            }
            ("uid", "OdooId") => {
                field_assigns.push(quote!(uid: self.auth.uid));
            }
            ("login", "String") => {
                field_assigns.push(quote!(login: self.auth.login.clone()));
            }
            ("password", "String") => {
                field_assigns.push(quote!(password: self.auth.password.clone()));
            }

            // strings are passed by ref
            //TODO: Into<String> would be more performant in some cases
            (_, "String") => {
                field_assigns.push(quote!(#ident: #ident.into()));
                field_arguments.push(quote!(#ident: &str));
            }

            // all other fields are passed as-is
            (_, _) => {
                field_assigns.push(quote!(#ident: #ident));
                field_arguments.push(quote!(#ident: #ty));
            }
        }
    }

    Ok(quote! {
        #[cfg(not(feature = "types-only"))]
        #[doc=#doc]
        impl<I: odoo_api::client::RequestImpl, #auth_generic> odoo_api::client::OdooClient<#auth_type, I> {
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

fn impl_serialize(
    ident_struct: &Ident,
    args: &OdooOrmArgs,
) -> Result<TokenStream2> {

    let ident_args: Vec<Ident> = args.args.iter().map(|x| Ident::new(x, Span::call_site())).collect();
    let lit_kwargs = args.kwargs.clone();
    let ident_kwargs: Vec<Ident> = args.kwargs.iter().map(|x| Ident::new(x, Span::call_site())).collect();
    // let len_kwargs = args.kwargs.len();
    Ok(quote!(
        impl serde::Serialize for #ident_struct {
            fn serialize<S>(&self, serialize: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer
            {
                let mut state = serialize.serialize_tuple(5)?;
                state.serialize_element(&self.database)?;
                state.serialize_element(&self.uid)?;
                state.serialize_element(&self.password)?;

                //TODO: serialize these directly (serialize.clone() ?)
                state.serialize_element(&(
                    ::serde_json::json!([
                        #(&self.#ident_args),*
                    ])
                ))?;

                //TODO: serialize these directly (serialize.clone() ?)
                state.serialize_element(&(
                    ::serde_json::json!({
                        #(#lit_kwargs : &self.#ident_kwargs),*
                    })
                ))?;

                state.end()
            }
        }
    ))
}