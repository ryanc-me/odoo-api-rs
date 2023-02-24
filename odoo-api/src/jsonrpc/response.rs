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
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponse<T>
where
    T: Debug,
{
    Success(JsonRpcResponseSuccess<T>),
    Error(JsonRpcResponseError),
}

/// A successful Odoo API response
#[derive(Debug, Serialize, Deserialize)]
pub struct JsonRpcResponseSuccess<T>
where
    //TODO: should we have something else here?
    T: Debug,
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
#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
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

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JsonRpcError {}

#[derive(Debug, Serialize, Deserialize)]
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
