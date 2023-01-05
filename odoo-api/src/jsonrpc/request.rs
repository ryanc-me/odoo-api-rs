//! JSON-RPC Requests

use super::{JsonRpcId, JsonRpcMethod, JsonRpcVersion};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

pub use api::{OdooApiContainer, OdooApiMethod};
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

mod api {
    use crate::jsonrpc::JsonRpcId;

    use super::{JsonRpcMethod, JsonRpcParams, JsonRpcRequest, JsonRpcVersion};
    use serde::ser::{SerializeStruct, Serializer};
    use serde::Serialize;
    use std::fmt::Debug;

    /// The container type for an Odoo "API" (JSON-RPC) request
    ///
    /// For more info, see [`super::JsonRpcParams`]
    #[derive(Debug)]
    pub struct OdooApiContainer<T>
    where
        T: OdooApiMethod + JsonRpcParams<Container<T> = Self>,
    {
        pub(crate) inner: T,
    }

    // Custom "man-in-the-middle" serialize impl
    impl<T> Serialize for OdooApiContainer<T>
    where
        T: OdooApiMethod + JsonRpcParams<Container<T> = Self>,
    {
        fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("args", 3)?;
            let (service, method) = self.inner.describe();
            state.serialize_field("service", service)?;
            state.serialize_field("method", method)?;
            state.serialize_field("args", &self.inner)?;
            state.end()
        }
    }

    /// An Odoo "API" (JSON-RPC) request type
    pub trait OdooApiMethod
    where
        Self: Sized + Debug + Serialize + JsonRpcParams<Container<Self> = OdooApiContainer<Self>>,
        Self::Container<Self>: Debug + Serialize,
    {
        /// Describe the JSON-RPC service and method for this type
        fn describe(&self) -> (&'static str, &'static str);

        /// Build `self` into a full [`JsonRpcRequest`]
        fn _build(self, id: JsonRpcId) -> JsonRpcRequest<Self> {
            JsonRpcRequest {
                jsonrpc: JsonRpcVersion::V2,
                method: JsonRpcMethod::Call,
                id,
                params: OdooApiContainer { inner: self },
            }
        }
    }
}

mod web {
    use serde::Serialize;
    use std::fmt::Debug;

    use super::{JsonRpcId, JsonRpcMethod, JsonRpcParams, JsonRpcRequest, JsonRpcVersion};

    /// The container type for an Odoo "Web" request
    ///
    /// This type covers (almost) any request whose endpoint starts with `/web`,
    /// for example:
    ///  - `/web/session/authenticate`
    ///  - `/web/session/destroy`
    ///  - `/web/dataset/call`
    ///  - And many more
    ///
    /// For more info, see [`super::JsonRpcParams`]
    #[derive(Debug, Serialize)]
    #[serde(transparent)]
    pub struct OdooWebContainer<T>
    where
        T: OdooWebMethod + JsonRpcParams<Container<T> = Self>,
    {
        pub(crate) inner: T,
    }

    /// An Odoo "Web" request type
    pub trait OdooWebMethod
    where
        Self: Sized + Debug + Serialize + JsonRpcParams<Container<Self> = OdooWebContainer<Self>>,
        Self::Container<Self>: Debug + Serialize,
    {
        /// Describe the "Web" method endpoint (e.g., "/web/session/authenticate")
        fn describe(&self) -> &'static str;

        /// Build `self` into a full [`JsonRpcRequest`]
        fn _build(self, id: JsonRpcId) -> JsonRpcRequest<Self> {
            JsonRpcRequest {
                jsonrpc: JsonRpcVersion::V2,
                method: JsonRpcMethod::Call,
                id,
                params: OdooWebContainer { inner: self },
            }
        }
    }
}
