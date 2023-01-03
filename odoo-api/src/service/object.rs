//! The Odoo "object" service (JSON-RPC)
//!
//! This service provides low-level methods to interact with Odoo models (`execute`
//! and `execute_kw`).
//!
//! For higher-level methods (e.g., `read` and `search_read`), see [`crate::service::orm`]

use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use serde_tuple::{Serialize_tuple};
use odoo_api_macros::{odoo_api};
use crate::jsonrpc::{OdooApiMethod, OdooId};
use crate as odoo_api;


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
#[odoo_api(
    service = "object",
    method = "execute",
    auth = true,
)]
#[derive(Debug, Serialize_tuple, PartialEq)]
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
    model: String,

    /// The method name (e.g. "read" or "create")
    method: String,

    /// The arguments (*args)
    args: Vec<Value>,
}

/// Represents the response to an Odoo [`Execute`]
///
/// This struct is intentionally very generic, as the `execute` call can return
/// any arbitrary JSON data.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ExecuteResponse {
    data: Value
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
#[odoo_api(
    service = "object",
    method = "execute_kw",
    auth = true,
)]
#[derive(Debug, Serialize_tuple, PartialEq)]
pub struct ExecuteKw {
    /// The database name
    database: String,

    /// The user id
    uid: OdooId,

    /// The user password
    password: String,

    /// The model name
    model: String,

    /// The method name (e.g. "read" or "create")
    method: String,

    /// The arguments (*args)
    args: Vec<Value>,

    /// The keyword-argments (**kwargs)
    kwargs: Map<String, Value>,
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

// #[cfg(test)]
// mod test {
//     use serde_json::{json, to_value, Value, Map};
//     use super::{Execute};
//     use crate::jsonrpc::{OdooApiMethod};
//     use crate::jvec;

//     #[test]
//     fn execute() -> Result<(), Box<dyn std::error::Error>> {
//         let request = Execute {
//             database: "my-database".into(),
//             uid: 2,
//             password: "Password1".into(),

//             model: "res.users".into(),
//             method: "read".into(),
//             args: jvec![],

//         }._build();
//         let request_string = to_value(&request)?;
//         let expected_string = json!({
//             "jsonrpc": "2.0",
//             "method": "call",
//             "id": 1000,
//             "params": {
//               "service": "object",
//               "method": "execute",
//               "args": [
//                 "my-database",
//                 2,
//                 "Password1",
//                 "res.users",
//                 "read",
//                 []
//               ]
//             }
//         });

//         Ok(())
//     }
// }