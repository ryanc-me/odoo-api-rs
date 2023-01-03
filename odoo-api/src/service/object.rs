//! The Odoo "object" service (JSON-RPC)
//!
//! This service provides low-level methods to interact with Odoo models (`execute`
//! and `execute_kw`).
//!
//! For higher-level methods (e.g., `read` and `search_read`), see [`crate::service::orm`]

use crate as odoo_api;
use crate::jsonrpc::{OdooApiMethod, OdooId};
use odoo_api_macros::odoo_api;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_tuple::Serialize_tuple;

/// Call a business-logic method on an Odoo model (positional args)
///
/// This method allows you to call an arbitrary Odoo method (e.g. `read` or
/// `create` or `my_function`), passing some arbitrary data, and returns the
/// result of that method call.
///
/// Note that the way this method handles keyword argument is unintuitive. If
/// you need to send `kwargs` to an Odoo method, you should use [`ExecuteKw`]
/// instead
///
/// See: [odoo/service/model.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/model.py#L62-L68)
#[odoo_api(service = "object", method = "execute", auth = true)]
#[derive(Debug, PartialEq)]
pub struct Execute {
    /// The database name
    // #[odoo(auth="database")]
    database: String,

    /// The user id
    // #[odoo(auth="database")]
    uid: OdooId,

    /// The user password
    // #[odoo(auth="database")]
    password: String,

    /// The model name
    pub model: String,

    /// The method name (e.g. "read" or "create")
    pub method: String,

    /// The arguments (*args)
    pub args: Vec<Value>,
}

// execute is a special case: each element of the `args` field must be serialized
// as a sibling of the `model`/`method`/etc fields.
//
// so the final result looks like this:
//
// ```
// "args": [
//      database,
//      uid,
//      password,
//      model,
//      method
//      args[1],
//      args[2],
//      args[3]
//      ...
// ]
// ```
//
// also note that Execute needs to be serialized as a tuple, not an object
impl Serialize for Execute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_tuple(5 + self.args.len())?;
        state.serialize_element(&self.database)?;
        state.serialize_element(&self.uid)?;
        state.serialize_element(&self.password)?;
        state.serialize_element(&self.model)?;
        state.serialize_element(&self.method)?;
        for arg in &self.args {
            state.serialize_element(&arg)?;
        }

        state.end()
    }
}

/// Represents the response to an Odoo [`Execute`]
///
/// This struct is intentionally very generic, as the `execute` call can return
/// any arbitrary JSON data.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ExecuteResponse {
    pub data: Value,
}

/// Call a business-logic method on an Odoo model (positional & keyword args)
///
/// This method is very similar to `execute`; It allows you to call an arbitrary
/// Odoo method (e.g. `read` or `create` or `my_function`), passing some arbitrary
/// data, and returns the result of that method call.
///
/// This differs from `execute` in that keyword args (`kwargs`) can be passed.
///
///
/// Reference: [odoo/service/model.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/model.py#L58-L59)
#[odoo_api(service = "object", method = "execute_kw", auth = true)]
#[derive(Debug, Serialize_tuple, PartialEq)]
pub struct ExecuteKw {
    /// The database name
    database: String,

    /// The user id
    uid: OdooId,

    /// The user password
    password: String,

    /// The model name
    pub model: String,

    /// The method name (e.g. "read" or "create")
    pub method: String,

    /// The arguments (*args)
    pub args: Vec<Value>,

    /// The keyword-argments (**kwargs)
    pub kwargs: Map<String, Value>,
}

/// Represents the response to an Odoo [`Execute`] call
///
/// This struct is intentionally very generic, as the `execute` call can return
/// any arbitrary JSON data.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ExecuteKwResponse {
    pub data: Value,
}
