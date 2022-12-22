//! The Odoo "object" service (types only)

use odoo_api_macros::odoo_api_request;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

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
#[odoo_api_request(
    "object", "execute",
    "Call a business-logic method on an Odoo model (positional args)"
)]
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

/// Represents the response to an Odoo [`Execute`]
///
/// This struct is intentionally very generic, as the `execute` call can return
/// any arbitrary JSON data.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ExecuteResponse {
    pub data: Value,
}

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
#[odoo_api_request(
    "object", "execute_kw",
    "Call a business-logic method on an Odoo model (positional & keyword args)"
)]
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

/// Represents the response to an Odoo [`Execute`] call
///
/// This struct is intentionally very generic, as the `execute` call can return
/// any arbitrary JSON data.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ExecuteKwResponse {
    pub data: Value,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::jsonrpc::{JsonRpcResponseSuccess, JsonRpcVersion, OdooApiResponse, Result};
    use serde_json::{json, to_value};

    #[test]
    fn execute() -> Result<()> {
        let expected_request = to_value(json!({
            "version": "2.0",
            "id": 1000,
            "method": "call",
            "params": {
                "service": "object",
                "method": "execute",
                "args": [
                    "my-database",
                    1,
                    "password123",
                    "res.users",
                    "search",
                    [
                        ["login", "ilike", "%"],
                        "|",
                        ["active", "=", true],
                        ["active", "=", false]
                    ]
                ]
            }
        }))?;
        let expected_response = to_value(json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                1,
                2,
                3
            ]
        }))?;

        let request = super::execute(
            "my-database",
            1,
            "password123",
            "res.users",
            "search",
            json!([
                ["login", "ilike", "%"],
                "|",
                ["active", "=", true],
                ["active", "=", false]
            ]),
        )?
        .to_json_value()?;

        let response = to_value(OdooApiResponse::<Execute>::Success(
            JsonRpcResponseSuccess {
                jsonrpc: JsonRpcVersion::V2,
                id: 1000,
                result: ExecuteResponse {
                    data: json!([1, 2, 3]),
                },
            },
        ))?;

        assert_eq!(request, expected_request);
        assert_eq!(response, expected_response);

        Ok(())
    }

    #[test]
    fn execute_kw() -> Result<()> {
        let expected_request = to_value(json!({
            "version": "2.0",
            "id": 1000,
            "method": "call",
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "my-database",
                    1,
                    "password123",
                    "res.users",
                    "search",
                    [
                        [
                            ["login", "ilike", "%"],
                            "|",
                            ["active", "=", true],
                            ["active", "=", false]
                        ]
                    ],
                    {
                        "limit": 1
                    }
                ]
            }
        }))?;
        let expected_response = to_value(json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                1
            ]
        }))?;

        let request = super::execute_kw(
            "my-database",
            1,
            "password123",
            "res.users",
            "search",
            json!([[
                ["login", "ilike", "%"],
                "|",
                ["active", "=", true],
                ["active", "=", false]
            ]]),
            json!({
                "limit": 1
            }),
        )?
        .to_json_value()?;

        let response = to_value(OdooApiResponse::<ExecuteKw>::Success(
            JsonRpcResponseSuccess {
                jsonrpc: JsonRpcVersion::V2,
                id: 1000,
                result: ExecuteKwResponse { data: json!([1]) },
            },
        ))?;

        assert_eq!(request, expected_request);
        assert_eq!(response, expected_response);

        Ok(())
    }
}
