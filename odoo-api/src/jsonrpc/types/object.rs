//! The Odoo "object" service (types only)

use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use odoo_api_macros::odoo_api_request;


/// Represents an **`object/execute`** API call.
///
/// **Service**: `object`  
/// **Method**: `execute`  
/// **Request**: [`Execute`]  
/// **Returns**: [`ExecuteResponse`]  
///
/// This method  allows you to call an arbitrary Odoo method (e.g. `read` or
/// `create` or `my_function`), passing some arbitrary data, and returns the
/// result of that method call.
///
/// Note that this method does not support keyword args. If you need to pass
/// kwargs, see [`ExecuteKw`] and [`execute_kw`].
///
/// Example:
/// ```no_run
/// use odoo_api::types::object;
/// use serde_json::json;
///
/// let request = object::execute(
///     "my-database",
///     1, // admin user
///     "password1",
///     "res.users",
///     "read",
///     json!([
///         [1, 2, 3],
///         ["name", "login"]
///     ])
/// );
/// ```
///
/// See: [odoo/service/model.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/model.py#L62-L68)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("object", "execute")]
pub struct Execute {
    /// The database name
    db: String,

    /// The user id
    uid: u32,

    /// The user password
    password: String,

    /// The model name
    model: String,

    /// The method name (e.g. "read" or "create")
    method: String,

    /// The arguments (*args)
    args: Vec<Value>,
}

/// Represents the response to an Odoo [`Execute`] call.
///
/// This struct is intentionally very generic, as the `execute` call can return
/// any arbitrary JSON data.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ExecuteResponse {
    pub data: Value
}



/// Represents an **`object/execute_kw`** API call.
///
/// **Service**: `object`  
/// **Method**: `execute_kw`  
/// **Request**: [`ExecuteKw`]  
/// **Returns**: [`ExecuteKwResponse`]  
///
/// This method is very similar to `execute`; It allows you to call an arbitrary
/// Odoo method (e.g. `read` or `create` or `my_function`), passing some arbitrary
/// data, and returns the result of that method call.
///
/// This differs from `execute` in that keyword args (`kwargs`) can be passed.
///
/// Example:
/// ```no_run
/// use odoo_api::types::object;
/// use serde_json::json;
///
/// let request = object::execute_kw(
///     "my-database",
///     1, // admin user
///     "password1",
///     "res.users",
///     "read",
///     json!([]),
///     json!({
///         "limit": 5,
///     })
/// );
/// ```
///
/// Reference: [odoo/service/model.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/model.py#L58-L59)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("object", "execute_kw")]
pub struct ExecuteKw {
    /// The database name
    db: String,

    /// The user id
    uid: u32,

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

/// Represents the response to an Odoo [`Execute`] call.
///
/// This struct is intentionally very generic, as the `execute` call can return
/// any arbitrary JSON data.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ExecuteKwResponse {
    pub data: Value
}

