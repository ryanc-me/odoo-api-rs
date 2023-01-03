//! The base JSON-RPC types
//!
//! This module exposes type structs, traits, and helper methods to build valid
//! Odoo JSON-RPC requests.
//!
//! As a crate user, you shouldn't need to interact with these directly. Instead, see [`crate::client`].

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub use request::{
    JsonRpcParams, JsonRpcRequest, OdooApiContainer, OdooApiMethod, OdooWebContainer, OdooWebMethod,
};
pub use response::JsonRpcResponse;

/// A JSON-RPC request id
pub type JsonRpcId = u32;

/// An Odoo record id
///
/// Note that this *is* signed, as some Odoo models (e.g. the `PurchaseBillUnion`)
/// use positive ids to represent one model (`purchase.order`), and negative ids
/// to represent another (`account.move`).
pub type OdooId = i32;

/// A string representing the JSON-RPC version
///
/// At the time of writing, this is always set to "2.0"
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
    /// Odoo JSON-RCP API version 2.0
    #[serde(rename = "2.0")]
    V2,
}

/// A string representing the JSON-RPC "method"
///
/// At the time of writing, this is always set to "call"
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcMethod {
    #[serde(rename = "call")]
    Call,
}

pub mod request {
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

        fn build(self) -> JsonRpcRequest<Self>;
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
            Self:
                Sized + Debug + Serialize + JsonRpcParams<Container<Self> = OdooApiContainer<Self>>,
            Self::Container<Self>: Debug + Serialize,
        {
            /// Describe the JSON-RPC service and method for this type
            fn describe(&self) -> (&'static str, &'static str);

            /// Build `self` into a full [`JsonRpcRequest`]
            fn _build(self) -> JsonRpcRequest<Self> {
                let id = {
                    #[cfg(test)]
                    {
                        // use a known id when testing
                        1000
                    }
                    #[cfg(not(test))]
                    {
                        1000
                    }
                };
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

        use super::{JsonRpcMethod, JsonRpcParams, JsonRpcRequest, JsonRpcVersion};

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
            Self:
                Sized + Debug + Serialize + JsonRpcParams<Container<Self> = OdooWebContainer<Self>>,
            Self::Container<Self>: Debug + Serialize,
        {
            /// Describe the "Web" method endpoint (e.g., "/web/session/authenticate")
            fn describe(&self) -> &'static str;

            /// Build `self` into a full [`JsonRpcRequest`]
            fn _build(self) -> JsonRpcRequest<Self> {
                JsonRpcRequest {
                    jsonrpc: JsonRpcVersion::V2,
                    method: JsonRpcMethod::Call,
                    id: 1000,
                    params: OdooWebContainer { inner: self },
                }
            }
        }
    }
}

pub mod response {
    //! JSON-RPC Responses

    use super::{JsonRpcId, JsonRpcVersion};
    use serde::{Deserialize, Serialize};
    use serde_json::{Map, Value};
    use std::fmt::Debug;

    /// An Odoo JSON-RPC API response
    ///
    /// This struct represents the base JSON data, and is paramterized over the
    /// *request* [`OdooApiMethod`](super::OdooApiMethod). The deserialization struct is chosen by
    /// looking at the associated type [`OdooApiMethod::Response`](super::OdooApiMethod).
    ///
    /// See: [odoo/http.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/http.py#L1805-L1841)
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(untagged)]
    pub enum JsonRpcResponse<T>
    where
        T: Debug,
    {
        Success(JsonRpcResponseSuccess<T>),
        Error(JsonRpcResponseError),
    }

    /// A successful Odoo API response
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct JsonRpcResponseSuccess<T>
    where
        T:,
    {
        /// The JSON-RPC version (`2.0`)
        pub(crate) jsonrpc: JsonRpcVersion,

        /// The request id
        ///
        /// This is not used for any stateful behaviour on the Odoo/Python side
        pub(crate) id: JsonRpcId,

        /// The response data, parameterized on the *request* [`OdooApiMethod::Response`](super::OdooApiMethod)
        /// associated type.
        pub(crate) result: T,
    }

    /// A failed Odoo API response
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct JsonRpcResponseError {
        /// The JSON-RPC version (`2.0`)
        pub(crate) jsonrpc: JsonRpcVersion,

        /// The request id
        ///
        /// This is not used for any stateful behaviour on the Odoo/Python side
        pub(crate) id: JsonRpcId,

        /// A struct containing the error information
        pub(crate) error: JsonRpcError,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct JsonRpcError {
        /// The error code. Currently hardcoded to `200`
        pub code: u32,

        /// The error "message". This is a short string indicating the type of
        /// error. Some examples are:
        ///  * `Odoo Server Error`
        ///  * `404: Not Found`
        ///  * `Odoo Session Expired`
        pub message: String,

        /// The actual error data
        pub data: JsonRpcErrorData,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub struct JsonRpcErrorData {
        /// The module? and type of the object where the exception was raised
        ///
        /// For example:
        ///  * `builtins.TypeError`
        ///  * `odoo.addons.account.models.account_move.AccountMove`
        pub name: String,

        /// The Python exception stack trace
        pub debug: String,

        /// The Python exception message (e.g. `str(exception)`)
        pub message: String,

        /// The Python exception arguments (e.g. `excetion.args`)
        pub arguments: Vec<Value>,

        /// The Python exception context (e.g. `excetion.context`)
        pub context: Map<String, Value>,
    }
}
