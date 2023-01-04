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
/// ## Example
/// ```no_run
/// # use odoo_api::OdooClient;
/// # let client = OdooClient::new_reqwest_blocking("").unwrap();
/// # let client = client.authenticate_manual("", "", 1, "", None);
/// use odoo_api::jvec;
///
/// // read `id` and `login` from users id=1,2,3
/// client.execute(
///     "res.users",
///     "read",
///     jvec![
///         [1, 2, 3],
///         ["id", "login"]
///     ]
/// );
/// ```
///
/// ## Arguments
///
/// ### `method`
///
/// The `method` field indicates the Python function to be called. This can be
/// any non-private method. Methods starting with an underscore (e.g. `_onchange_name`)
/// are considered to be "private".
///
/// ### `args`
///
/// The arguments are passed to Python as `object.method_name(*args)`, so
/// kwargs are technically supported here.
///
/// For example, consider the Python function
/// ```python
/// def search_read(domain, fields=None):
///     pass
/// ```
///
/// Our `args` field should be structured like:
/// ```no_run
/// # use odoo_api::jvec;
/// let args = jvec![
///     // element #1 goes to `domain`
///     [
///         ["name", "!=", "admin"],
///     ],
///
///     // element #2 goes to `fields`
///     ["id", "login"]
/// ];
/// ```
///
/// <br />
///
/// Also note that many Odoo methods accept `self` as the first param. In that
/// case, you should pass a list of IDs as the first element.
///
/// See: [odoo/service/model.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/model.py#L62-L68)
#[odoo_api(service = "object", method = "execute", auth = true)]
#[derive(Debug)]
pub struct Execute {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The model name
    pub model: String,

    /// The method name
    pub method: String,

    /// The method arguments
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
#[derive(Debug, Serialize, Deserialize)]
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
/// ## Execute:
/// ```no_run
/// # use odoo_api::OdooClient;
/// # let client = OdooClient::new_reqwest_blocking("").unwrap();
/// # let client = client.authenticate_manual("", "", 1, "", None);
/// use odoo_api::{jvec, jmap};
///
/// // read `id` and `login` from any user whose email matches "%@example.com"
/// client.execute_kw(
///     "res.users",
///     "search_read",
///     jvec![
///         [["login", "=ilike", "%@example.com"]]
///     ],
///     jmap!{
///         "fields": ["id", "login"]
///     }
/// );
/// ```
///
/// <br />
///
/// ## Arguments
///
/// ### `method`
///
/// The `method` field indicates the Python function to be called. This can be
/// any non-private method. Methods starting with an underscore (e.g. `_onchange_name`)
/// are considered to be "private".
///
/// ### `args` and `kwargs`
///
/// The method args (position and keyword) are passed to Python as `(*args, **kwargs)`.
///
/// For example:
/// ```python
/// ## this function...
/// def search_read(self, domain, fields=None):
///     pass
///
/// ## ...would be called like
/// model.search_read(*args, **kwargs)
/// ```
///
/// This is much simpler than [`Execute`].
///
/// Also note that many Odoo methods accept `self` as the first param. In that
/// case, you should pass a list of IDs as the first element.
///
///
/// Reference: [odoo/service/model.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/model.py#L58-L59)
#[odoo_api(service = "object", method = "execute_kw", auth = true)]
#[derive(Debug, Serialize_tuple)]
pub struct ExecuteKw {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The model name
    pub model: String,

    /// The method name
    pub method: String,

    /// The positional arguments
    pub args: Vec<Value>,

    /// The keyword argments
    pub kwargs: Map<String, Value>,
}

/// Represents the response to an Odoo [`Execute`] call
///
/// This struct is intentionally very generic, as the `execute` call can return
/// any arbitrary JSON data.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExecuteKwResponse {
    pub data: Value,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::jsonrpc::{JsonRpcParams, JsonRpcResponse};
    use crate::{jmap, jvec, Result};
    use serde_json::{from_value, json, to_value};

    /// Test that serializing the [`Execute`] struct produces the expected
    /// JSON output.
    ///
    /// This is important because we're *always* using named-field structs on
    /// the Rust side (for convenience), but several API methods actually
    /// expect lists of values.
    ///
    /// Additionally, for Execute, the `args` field is serialized as a sibling
    /// to the other fields (see the `impl Serialize` above for more info),
    ///
    ///
    /// We'll follow this test pattern for all other API methods:
    ///  - Build a valid JSON payload in Postman, using a real production Odoo 14.0+e instance
    ///  - That JSON payload becomes the `expected` variable
    ///  - Build the request struct in the test function (`execute` variable below)
    ///  - Compare the two with `assert_eq!()`
    ///
    /// This should ensure that the crate is producing valid JSON payloads
    #[test]
    fn execute() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.users",
                    "read",
                    [1, 2],
                    ["id", "login"]
                ]
            }
        });
        let actual = to_value(
            Execute {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.users".into(),
                method: "read".into(),
                args: jvec![[1, 2], ["id", "login"]],
            }
            .build(),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    /// Test that a valid Odoo response payload is serializable into [`ExecuteResponse`]
    ///
    /// As with [`execute`] above, this is achieved by firing a JSON-RPC request
    /// at a live Odoo instance. Here we take the response JSON and try to serialize
    /// it into the [`ExecuteResponse`] struct via `from_value()`.
    ///
    /// If this succeeds, then the response struct is set up properly!
    #[test]
    fn execute_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                {
                    "id": 1,
                    "login": "__system__"
                },
                {
                    "id": 2,
                    "login": "admin"
                }
            ]
        });

        let response: JsonRpcResponse<ExecuteResponse> = from_value(payload)?;

        // note that this match isn't strictly necessary right now, because
        // the Error() variant is only produced when the input JSON contains
        // an `"error": {}` key (and we aren't testing those cases).
        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    /// See [`crate::service::object::test::execute`] for more info
    #[test]
    fn execute_kw() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.users",
                    "read",
                    [
                        [1, 2]
                    ],
                    {
                        "fields": ["id", "login"]
                    }
                ]
            }
        });
        let actual = to_value(
            ExecuteKw {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.users".into(),
                method: "read".into(),
                args: jvec![[1, 2]],
                kwargs: jmap! {
                    "fields": ["id", "login"]
                },
            }
            .build(),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    /// See [`crate::service::object::test::execute_response`] for more info
    #[test]
    fn execute_kw_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                {
                    "id": 1,
                    "login": "__system__"
                },
                {
                    "id": 2,
                    "login": "admin"
                }
            ]
        });

        let response: JsonRpcResponse<ExecuteKwResponse> = from_value(payload)?;
        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }
}
