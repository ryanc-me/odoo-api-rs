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
