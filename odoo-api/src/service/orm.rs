//! The Odoo "ORM" pseudo-service
//!
//! This isn't really an Odoo "service", but instead is another layer of abstraction
//! over the object `execute` and `execute_kw` methods, providing a nicer interface
//! with better type checking.

use crate as odoo_api;
use crate::jsonrpc::{OdooId, OdooIds, OdooOrmMethod};
use odoo_api_macros::odoo_orm;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};
use serde_json::{Value, Map};

/// Create a new record (or set of records)
///
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::OdooClient;
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// // note that auth fields (db, login, password) are auto-filled
/// // for you by the client
/// let resp = client.create(
///     "res.partner",
///     jmap!{
///         "name": "Example Partner",
///         "email": "hello@example.com",
///     }
/// ).send()?;
///
/// println!("New partner ID(s): {}", resp.ids);
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3829-L3964)
#[odoo_orm(
    method = "create",
    args = ["values"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct Create {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// Values for the new record(s)
    pub values: CreateVals,
}

/// The values to a [`Create`] request
/// 
/// The Odoo `create()` function accepts either a dictionary (create 1x record),
/// or a list of dictionaries (create multiple records). To support those in an
/// ergonomic way, we will accept an enum for the value.
///
/// This enum implements `From<...>` for both one & multi requests:
/// ```ignore
/// // create a single record
/// client.create(
///     "res.users",
///     jmap!{
///         "name": "Hello, world!",
///     }
/// );
/// 
/// // create multiple records
/// client.create(
///     "res.users",
///     jvec![
///         {"name": "Partner #1"},
///         {"name": "Partner #2"}
///     ]
/// );
/// ```
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CreateVals {
    One(Map<String, Value>),
    Multi(Vec<Map<String, Value>>)
}

impl From<Map<String, Value>> for CreateVals {
    fn from(value: Map<String, Value>) -> Self {
        Self::One(value)
    }
}

impl From<Vec<Map<String, Value>>> for CreateVals {
    fn from(value: Vec<Map<String, Value>>) -> Self {
        Self::Multi(value)
    }
}

/// The response to a [`Create`] requests
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateResponse {
    /// The ID (or IDS) of the newly created records
    ids: CreateResponseItem,
}

/// Container for the [`CreateResponse`] items
/// 
/// Because thr [`Create`] request can create one OR multiple records, the response
/// may be one or multiple ids. In the "one" case, Odoo returns a plain int. In
/// the "multi" case, Odoo returns an array of ints.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateResponseItem {
    One(OdooId),
    Multi(Vec<OdooId>)
}

/// Read data from a record (or set of records)
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L2958-L2991)
#[odoo_orm(
    method = "read",
    args = ["ids"],
    kwargs = ["fields"],
)]
#[derive(Debug)]
pub struct Read {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records
    pub ids: OdooIds,

    /// The fields to be fetched
    pub fields: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadResponse {
    pub data: Vec<Map<String, Value>>
}

/// Write data to a record (or set of records)
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3585-L3775)
#[odoo_orm(
    method = "write",
    args = ["ids", "values"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct Write {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records
    pub ids: OdooIds,

    /// The values to write
    pub values: Map<String, Value>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WriteResponse {
    pub ok: bool
}

/// Delete a record (or set of records)
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3488-L3583)
#[odoo_orm(
    method = "unlink",
    args = ["ids"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct Unlink {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    //TODO: model method, this should always be blank
    pub ids: OdooIds,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UnlinkResponse {
    pub ok: bool
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::error::Result;
    use crate::jsonrpc::{JsonRpcParams, JsonRpcResponse};
    use crate::{jmap, jvec};
    use serde_json::{from_value, json, to_value};

    #[test]
    fn create() -> Result<()> {
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
                    "res.partner",
                    "create",
                    [
                        {"name": "Hello, world!"}
                    ],
                    {}
                ]
            }
        });
        let actual = to_value(
            Create {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                values: jmap!{"name": "Hello, world!"}.into(),
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn create_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": 47
        });

        let response: JsonRpcResponse<CreateResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn read() -> Result<()> {
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
                    "res.partner",
                    "read",
                    [
                        [1, 2, 3]
                    ],
                    {
                        "fields": ["id", "login"]
                    }
                ]
            }
        });
        let actual = to_value(
            Read {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                ids: vec![1, 2, 3].into(),
                fields: vec!["id".into(), "login".into()],
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn read_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                {
                    "id": 1,
                    "name": "My Company (San Francisco)"
                },
                {
                    "id": 2,
                    "name": "OdooBot"
                },
                {
                    "id": 3,
                    "name": "Administrator"
                }
            ]
        });

        let response: JsonRpcResponse<ReadResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn write() -> Result<()> {
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
                    "res.partner",
                    "write",
                    [
                        [2],
                        {
                            "name": "The Admin Account"
                        }
                    ],
                    {}
                ]
            }
        });
        let actual = to_value(
            Write {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                ids: 2.into(),
                values: jmap!{"name": "The Admin Account"},
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn write_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": true
        });

        let response: JsonRpcResponse<WriteResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn unlink() -> Result<()> {
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
                    "res.partner",
                    "unlink",
                    [
                        [3],
                    ],
                    {}
                ]
            }
        });
        let actual = to_value(
            Unlink {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                ids: 3.into(), // the "default" user - be careful!
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn unlink_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": true
        });

        let response: JsonRpcResponse<UnlinkResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }
}
