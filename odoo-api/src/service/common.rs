//! The Odoo "common" service (JSON-RPC)
//!
//! This service provides misc methods like `version` and `authenticate`.
//!
//! Note that the authentication methods (`login` and `authenticate`) are both "dumb";
//! that is, they do not work with Odoo's sessioning mechanism. The result is that
//! these methods will not work for non-JSON-RPC methods (e.g. "Web" methods), and
//! they will not handle multi-database Odoo deployments.

use crate as odoo_api;
use crate::jsonrpc::{OdooApiMethod, OdooId};
use odoo_api_macros::odoo_api;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use serde_tuple::Serialize_tuple;

/// Check the user credentials and return the user ID
///
/// This method performs a "login" to the Odoo server, and returns the corresponding
/// user ID (`uid`).
///
/// Note that the Odoo JSON-RPC API is stateless; there are no sessions or tokens,
/// each requests passes the password (or API key). Therefore, calling this method
/// "login" is a misnomer - it doesn't actually "login", just checks the credentials
/// and returns the ID.
///
/// See: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L19-L20)  
/// See also: [base/models/res_users.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/addons/base/models/res_users.py#L762-L787)
#[odoo_api(
    service = "common",
    method = "login",
    name = "common_login",
    auth = true
)]
#[derive(Debug, Serialize_tuple)]
pub struct Login {
    /// The database name
    pub db: String,

    /// The username (e.g., email)
    pub login: String,

    /// The user password
    pub password: String,
}

/// Represents the response to an Odoo [`Login`] call
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LoginResponse {
    pub uid: OdooId,
}

/// Check the user credentials and return the user ID (web)
///
/// This method performs a "login" to the Odoo server, and returns the corresponding
/// user ID (`uid`). It is identical to [`Login`], except that it accepts an extra
/// param `user_agent_env`, which is normally sent by the browser.
///
/// This method is inteded for browser-based API implementations. You should use [`Login`] instead.
///
/// See: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L22-L29)  
/// See also: [base/models/res_users.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/addons/base/models/res_users.py#L762-L787)
#[odoo_api(
    service = "common",
    method = "authenticate",
    name = "common_authenticate",
    auth = true
)]
#[derive(Debug, Serialize_tuple)]
pub struct Authenticate {
    /// The database name
    pub db: String,

    /// The username (e.g., email)
    pub login: String,

    /// The user password
    pub password: String,

    /// A mapping of user agent env entries
    pub user_agent_env: Map<String, Value>,
}

/// Represents the response to an Odoo [`Authenticate`] call
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AuthenticateResponse {
    pub uid: OdooId,
}

/// Fetch detailed information about the Odoo version
///
/// This method returns some information about the Odoo version (represented in
/// the [`ServerVersionInfo`] struct), along with some other metadata.
///
/// Odoo's versioning was inspired by Python's [`sys.version_info`](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/release.py#L11),
/// with an added field to indicate whether the server is running Enterprise or
/// Community edition. In practice, `minor` and `micro` are typically both `0`,
/// so an Odoo version looks something like: `14.0.0.final.0.e`
///
/// See: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L31-L32)  
/// See also: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L12-L17)  
/// See also: [odoo/release.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/release.py)
#[odoo_api(
    service = "common",
    method = "version",
    name = "common_version",
    auth = false
)]
#[derive(Debug)]
pub struct Version {}

// Version has no fields, but needs to output in JSON: `[]`
impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let state = serializer.serialize_tuple(0)?;
        state.end()
    }
}

/// Represents the response to an Odoo [`Version`] call
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionResponse {
    /// The "pretty" version, normally something like `16.0+e` or `15.0`
    pub server_version: String,

    /// The "full" version. See [`ServerVersionInfo`] for details
    pub server_version_info: ServerVersionInfo,

    /// The server "series"; like `server_version`, but without any indication of Enterprise vs Community (e.g., `16.0` or `15.0`)
    pub server_serie: String,

    /// The Odoo "protocol version". At the time of writing, it isn't clear where this is actually used, and `1` is always returned
    pub protocol_version: u32,
}

/// A struct representing the Odoo server version info
///
/// See: [odoo/services/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L12-L17)  
/// See also: [odoo/release.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/release.py)
#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct ServerVersionInfo {
    /// The "major" version (e.g., `16`)
    pub major: u32,

    /// The "minor" version (e.g., `0`)
    pub minor: u32,

    /// The "micro" version (e.g., `0`)
    pub micro: u32,

    /// The "release level"; one of `alpha`, `beta`, `candidate`, or `final`. For live servers, this is almost always `final`
    pub release_level: String,

    /// The release serial
    pub serial: u32,

    /// A string indicating whether Odoo is running in Enterprise or Community mode; `None` = Community, Some("e") = Enterprise
    pub enterprise: Option<String>,
}

/// Fetch basic information about the Odoo version
///
/// Returns a link to the old OpenERP website, and optionally the "basic" Odoo
/// version string (e.g. `16.0+e`).
///
/// This call isn't particularly useful on its own - you probably want to use [`Version`]
/// instead.
///
/// See: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L34-L45)  
/// See also: [odoo/release.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/release.py)
#[odoo_api(
    service = "common",
    method = "about",
    name = "common_about",
    auth = false
)]
#[derive(Debug, Serialize_tuple)]
pub struct About {
    pub extended: bool,
}

//TODO: flat deserializ so we can have either `result: "http://..."` or `result: ["http://..", "14.0+e"]`
/// Represents the response to an Odoo [`About`] call
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AboutResponse {
    /// Basic response; includes only the `info` string
    Basic(AboutResponseBasic),

    /// Extended response; includes `info` string and version info
    Extended(AboutResponseExtended),
}

/// Represents the response to an Odoo [`About`] call
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AboutResponseBasic {
    /// The "info" string
    ///
    /// At the time of writing, this is hard-coded to `See http://openerp.com`
    pub info: String,
}

/// Represents the response to an Odoo [`About`] call
#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct AboutResponseExtended {
    /// The "info" string
    ///
    /// At the time of writing, this is hard-coded to `See http://openerp.com`
    pub info: String,

    /// The "pretty" version, normally something like `16.0+e` or `15.0`
    ///
    /// Note that this is only returned when the original reques was made with
    /// `extended: true` (see [`AboutResponse`])
    pub server_version: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::jsonrpc::{JsonRpcParams, JsonRpcResponse};
    use crate::{jmap, Result};
    use serde_json::{from_value, json, to_value};

    /// See [`crate::service::object::test::execute`] for more info
    #[test]
    fn login() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "common",
                "method": "login",
                "args": [
                    "some-database",
                    "admin",
                    "password",
                ]
            }
        });
        let actual = to_value(
            Login {
                db: "some-database".into(),
                login: "admin".into(),
                password: "password".into(),
            }
            .build(),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    /// See [`crate::service::object::test::execute_response`] for more info
    #[test]
    fn login_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": 2
        });

        let response: JsonRpcResponse<LoginResponse> = from_value(payload)?;
        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    /// See [`crate::service::object::test::execute`] for more info
    #[test]
    fn authenticate() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "common",
                "method": "authenticate",
                "args": [
                    "some-database",
                    "admin",
                    "password",
                    {
                        "base_location": "https://demo.odoo.com"
                    }
                ]
            }
        });
        let actual = to_value(
            Authenticate {
                db: "some-database".into(),
                login: "admin".into(),
                password: "password".into(),
                user_agent_env: jmap! {
                    "base_location": "https://demo.odoo.com"
                },
            }
            .build(),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    /// See [`crate::service::object::test::execute_response`] for more info
    #[test]
    fn authenticate_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": 2
        });

        let response: JsonRpcResponse<AuthenticateResponse> = from_value(payload)?;
        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    /// See [`crate::service::object::test::execute`] for more info
    #[test]
    fn version() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "common",
                "method": "version",
                "args": []
            }
        });
        let actual = to_value(Version {}.build())?;

        assert_eq!(actual, expected);

        Ok(())
    }

    /// See [`crate::service::object::test::execute_response`] for more info
    #[test]
    fn version_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": {
                "server_version": "14.0+e",
                "server_version_info": [
                    14,
                    0,
                    0,
                    "final",
                    0,
                    "e"
                ],
                "server_serie": "14.0",
                "protocol_version": 1
            }
        });

        let response: JsonRpcResponse<VersionResponse> = from_value(payload)?;
        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    /// See [`crate::service::object::test::execute`] for more info
    #[test]
    fn about_basic() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "common",
                "method": "about",
                "args": [
                    false
                ]
            }
        });
        let actual = to_value(About { extended: false }.build())?;

        assert_eq!(actual, expected);

        Ok(())
    }

    /// See [`crate::service::object::test::execute_response`] for more info
    #[test]
    fn about_basic_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": "See http://openerp.com"
        });

        let response: JsonRpcResponse<AboutResponse> = from_value(payload)?;
        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(data) => match data.result {
                AboutResponse::Basic(_) => Ok(()),
                AboutResponse::Extended(_) => {
                    panic!("Expected the `Basic` response, but got `Extended`")
                }
            },
        }
    }

    /// See [`crate::service::object::test::execute`] for more info
    #[test]
    fn about_extended() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "common",
                "method": "about",
                "args": [
                    true
                ]
            }
        });
        let actual = to_value(About { extended: true }.build())?;

        assert_eq!(actual, expected);

        Ok(())
    }

    /// See [`crate::service::object::test::execute_response`] for more info
    #[test]
    fn about_extended_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                "See http://openerp.com",
                "14.0+e"
            ]
        });

        let response: JsonRpcResponse<AboutResponse> = from_value(payload)?;
        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(data) => match data.result {
                AboutResponse::Extended(_) => Ok(()),
                AboutResponse::Basic(_) => {
                    panic!("Expected the `Extended` response, but got `Basic`")
                }
            },
        }
    }
}
