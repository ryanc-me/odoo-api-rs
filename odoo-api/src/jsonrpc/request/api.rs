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

    /// Describe method endpoint (e.g., "/web/session/authenticate")
    fn endpoint(&self) -> &'static str;

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
