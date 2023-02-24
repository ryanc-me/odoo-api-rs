//! The base JSON-RPC types
//!
//! This module exposes type structs, traits, and helper methods to build valid
//! Odoo JSON-RPC requests.
//!
//! As a crate user, you shouldn't need to interact with these directly. Instead, see [`crate::client`].

pub mod request;
pub mod response;

use serde::{Deserialize, Serialize};
use std::fmt::Debug;

pub use request::{
    JsonRpcParams, JsonRpcRequest, OdooApiContainer, OdooApiMethod, OdooOrmContainer,
    OdooOrmMethod, OdooWebContainer, OdooWebMethod,
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

/// A vec of [`OdooId`].
///
/// This type also implements `From<OdooId>`, which allows for flexible function
/// args, e.g.:
/// ```
/// use odoo_api::jsonrpc::OdooIds;
/// fn my_function<I: Into<OdooIds>>(ids: I) {
///     // ...
/// }
///
/// // call with a list of ids...
/// my_function(vec![1, 2, 3]);
///
/// // ... or with a single id
/// my_function(1);
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct OdooIds(Vec<OdooId>);

impl From<OdooId> for OdooIds {
    fn from(value: OdooId) -> Self {
        OdooIds(vec![value])
    }
}
impl From<Vec<OdooId>> for OdooIds {
    fn from(value: Vec<OdooId>) -> Self {
        Self(value)
    }
}

/// A string representing the JSON-RPC version
///
/// At the time of writing, this is always set to "2.0"
#[derive(Debug, Serialize, Deserialize)]
pub enum JsonRpcVersion {
    /// Odoo JSON-RCP API version 2.0
    #[serde(rename = "2.0")]
    V2,
}

/// A string representing the JSON-RPC "method"
///
/// At the time of writing, this is always set to "call"
#[derive(Debug, Serialize, Deserialize)]
pub enum JsonRpcMethod {
    #[serde(rename = "call")]
    Call,
}
