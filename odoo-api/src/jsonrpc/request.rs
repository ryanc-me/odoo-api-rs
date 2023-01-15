//! JSON-RPC Requests

use super::{JsonRpcId, JsonRpcMethod, JsonRpcVersion};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

mod api;
mod orm;
mod web;

pub use api::{OdooApiContainer, OdooApiMethod};
pub use orm::{OdooOrmContainer, OdooOrmMethod};
pub use web::{OdooWebContainer, OdooWebMethod};

/// Implemented by Odoo "method" types (e.g.,
/// [`Execute`](crate::service::object::Execute) or
/// [`SessionAuthenticate`](crate::service::web::SessionAuthenticate))
///
/// When building an [`JsonRpcRequest`] object, the `params` field is actually
/// set to [`JsonRpcParams::Container`], not the concrete "method" type. This
/// allows for flexibility in the [`Serialize`] impl.
///
/// For example, the `Execute` method uses [`OdooApiContainer`] as its container,
/// which injects the `service` and `method` keys into the request struct:
/// ```json
/// {
///     "jsonrpc": "2.0",
///     "method": "call",
///     "id": 1000,
///     "params": {
///         "service": "object",
///         "method": "execute",
///         "args": <Execute is serialized here>
///     }
/// }
/// ```
///
/// Whereas the `SessionAuthenticate` method's container ([`OdooWebContainer`])
/// has a transparent Serialize impl, so the `SessionAuthenticate` data is set
/// directly on the `params` key:
/// ```json
/// {
///     "jsonrpc": "2.0",
///     "method": "call",
///     "id": 1000,
///     "params": <SessionAuthenticate is serialized here>
/// }
/// ```
pub trait JsonRpcParams
where
    Self: Sized + Debug + Serialize,
{
    type Container<T>: Debug + Serialize;
    type Response: Debug + DeserializeOwned;

    fn build(self, id: JsonRpcId) -> JsonRpcRequest<Self>;
}

/// A struct representing the full JSON-RPC request body
///
/// See [`JsonRpcParams`] for more info about the strange `params` field type.
#[derive(Debug, Serialize)]
pub struct JsonRpcRequest<T>
where
    T: JsonRpcParams + Serialize + Debug,
    T::Container<T>: Debug + Serialize,
{
    /// The JSON-RPC version (`2.0`)
    pub(crate) jsonrpc: JsonRpcVersion,

    /// The JSON-RPC method (`call`)
    pub(crate) method: JsonRpcMethod,

    /// The request id
    ///
    /// This is not used for any stateful behaviour on the Odoo/Python side
    pub(crate) id: JsonRpcId,

    /// The request params (service, method, and arguments)
    pub(crate) params: <T as JsonRpcParams>::Container<T>,
}
