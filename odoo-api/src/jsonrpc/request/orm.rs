use serde::ser::SerializeStruct;
use serde::{Serialize, Serializer};
use std::fmt::Debug;

use super::{JsonRpcId, JsonRpcMethod, JsonRpcParams, JsonRpcRequest, JsonRpcVersion};

/// The container type for an Odoo "ORM" request
///
/// These functions are essentially just wrappers around
/// [`Execute`](crate::service::object::Execute)
/// and  [`ExecuteKw`](crate::service::object::ExecuteKw), providing a more
/// user-friendly interface (and better type checking!)
///
/// For more info, see [`super::JsonRpcParams`]
#[derive(Debug)]
pub struct OdooOrmContainer<T>
where
    T: OdooOrmMethod + JsonRpcParams<Container<T> = Self>,
{
    pub(crate) inner: T,
}

// Custom "man-in-the-middle" serialize impl
impl<T> Serialize for OdooOrmContainer<T>
where
    T: OdooOrmMethod + JsonRpcParams<Container<T> = Self>,
{
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("args", 3)?;
        state.serialize_field("service", "object")?;
        state.serialize_field("method", "execute_kw")?;
        state.serialize_field("args", &self.inner)?;
        state.end()
    }
}

/// An Odoo "Orm" request type
pub trait OdooOrmMethod
where
    Self: Sized + Debug + Serialize + JsonRpcParams<Container<Self> = OdooOrmContainer<Self>>,
    Self::Container<Self>: Debug + Serialize,
{
    /// Describe the "ORM" method endpoint (e.g., "/web/session/authenticate")
    fn endpoint(&self) -> &'static str;

    /// Return the model method name (e.g., "read_group" or "create")
    fn method(&self) -> &'static str;

    /// Build `self` into a full [`JsonRpcRequest`]
    fn _build(self, id: JsonRpcId) -> JsonRpcRequest<Self> {
        JsonRpcRequest {
            jsonrpc: JsonRpcVersion::V2,
            method: JsonRpcMethod::Call,
            id,
            params: OdooOrmContainer { inner: self },
        }
    }
}
