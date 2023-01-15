//! Helper macros for the `odoo_api` crate
//!
//! See the [`odoo_api`](https://crates.io/crates/odoo-api) crate.

use proc_macro::TokenStream;

mod common;
mod error;
mod odoo_api;
mod odoo_web;
mod odoo_orm;
mod serialize_tuple;

use common::{parse_result};
use error::{Error, Result};
use syn::{parse_macro_input};

/// Implement traits for an "API" method struct
///
/// The input should be a struct with named field. Zero-sized structs and generics
/// are supported.
///
/// Arguments:
///  - Service: The Odoo "service" for this method
///  - Method: The method name
///  - Auth: Whether authentication is required, optional, or ignored
///
/// For example, consider the following:
/// ```ignore
/// // service: "object"
/// // method: "execute"
/// // auth: "yes"
/// #[derive(Debug, Serialize)]
/// #[odoo_api("object", "execute", "yes")]
/// struct Execute {
///     database: String,
///     uid: OdooId,
///     password: String,
///
///     model: String,
///     method: String,
///     args: Vec<Value>
/// }
/// ```
///
/// Then the following impls will be generated:
/// ```ignore
/// // The `Execute` is able to be used as JSON-RPC `params`
/// impl JsonRpcParams for Execute {
///     // Set the container; in this case, the container will use `describe` below
///     // to implement a custom Serialize
///     type Container<T> = OdooApiContainer<Self>;
///     type Response = ExecuteResponse;
///
///     fn build(self) -> JsonRpcRequest<Self> { self._build() }
/// }
///
/// // Which JSON-RPC service and method does `Execute` belong to?
/// impl OdooApiMethod for Execute {
///     fn describe(&self) -> (&'static str, &'static str) {
///         ("object", "execute")
///     }
/// }
///
/// // Implement a method for this struct, so users can write `client.execute(...)`
/// // Note that we specified "yes" for auth, so this impl is bound to `Authed`
/// // clients only
/// impl<I: RequestImpl> OdooClient<Authed, I> {
///     pub fn execute(&self, model: &str, method: &str, args: Vec<Value>) -> OdooRequest<Execute, I> {
///         let execute = Execute {
///             // Auth info is pulled from the Client
///             database: self.auth.database.clone(),
///             uid: self.auth.uid,
///             password: self.auth.password.clone(),
///
///             // Strings are passed as &str then converted to Strings
///             model: model.into(),
///             method: method.into(),
///             args
///         };
///
///         // Finally, build the request
///         self.build_request(
///             execute,
///             &self.url_jsonrpc
///         )
///     }
/// }
/// ```
#[proc_macro_attribute]
pub fn odoo_api(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args);
    let input = parse_macro_input!(input);

    parse_result(odoo_api::odoo_api(args, input))
}

#[proc_macro_attribute]
pub fn odoo_web(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args);
    let input = parse_macro_input!(input);

    parse_result(odoo_web::odoo_web(args, input))
}

#[proc_macro_attribute]
pub fn odoo_orm(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args);
    let input = parse_macro_input!(input);

    parse_result(odoo_orm::odoo_orm(args, input))
}

#[proc_macro_derive(SerializeTuple)]
pub fn serialize_tuple(input: TokenStream) -> TokenStream {
    parse_result(serialize_tuple::serialize_tuple(input))
}
