//! The Odoo "common" service (JSON-RPC)
//!
//! This service provides misc methods like `version` and `authenticate`.
//!
//! Note that the authentication methods (`login` and `authenticate`) are both "dumb";
//! that is, they do not work with Odoo's sessioning mechanism. The result is that
//! these methods will not work for non-JSON-RPC methods (e.g. "Web" methods), and
//! they will not handle multi-database Odoo deployments.

use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use serde_tuple::{Serialize_tuple};
use odoo_api_macros::{odoo_api};
use crate::jsonrpc::{OdooApiMethod, OdooId};
use crate as odoo_api;


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
/// Example:
/// ```no_run
/// use odoo_api::types::common;
///
/// let request = common::login(
///     "my-database",
///     "user@example.com",
///     "password1",
/// );
/// ```
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
/// user ID (`uid`). It is identical to [`login`], except that it accepts an extra
/// param `user_agent_env`, which is normally sent by the browser.
///
/// This method is inteded for browser-based API implementations. You should use [`Login`] or [`login`] instead.
///
/// Example:
/// ```no_run
/// use serde_json::{json, Map, Value};
/// use odoo_api::types::common;
///
/// let request = common::authenticate(
///     "my-database",
///     "user@example.com",
///     "password1",
///     json!({
///         "base_location": "demo.odoo.com"
///     })
/// );
/// ```
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
/// so an Odoo version looks something like:
///
/// 14.0.0.final.0.e
///
/// Example:
/// ```no_run
/// use odoo_api::types::common;
///
/// let request = common::version();
/// ```
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
#[derive(Debug, Serialize)]
pub struct Version {}

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
#[derive(Debug, Serialize_tuple, Deserialize, PartialEq)]
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
/// or [`version`] instead.
///
/// Example:
/// ```no_run
/// use odoo_api::types::common;
///
/// let request = common::about(true);
/// ```
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

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::jsonrpc::response::{OdooApiResponse, JsonRpcResponseSuccess};
//     use crate::jsonrpc::{JsonRpcVersion, Result};
//     use serde_json::{json, to_value};

//     #[test]
//     fn login() -> Result<()> {
//         let expected_request = to_value(json!({
//             "version": "2.0",
//             "id": 1000,
//             "method": "call",
//             "params": {
//                 "service": "common",
//                 "method": "login",
//                 "args": [
//                     "my-database",
//                     "admin",
//                     "password123"
//                 ]
//             }
//         }))?;
//         let expected_response = to_value(json!({
//             "jsonrpc": "2.0",
//             "id": 1000,
//             "result": 2
//         }))?;

//         let request = super::login("my-database", "admin", "password123")?.to_json_value()?;

//         let response = to_value(OdooApiResponse::<Login>::Success(JsonRpcResponseSuccess {
//             jsonrpc: JsonRpcVersion::V2,
//             id: 1000,
//             result: LoginResponse { uid: 2 },
//         }))?;

//         assert_eq!(request, expected_request);
//         assert_eq!(response, expected_response);

//         Ok(())
//     }

//     #[test]
//     fn authenticate() -> Result<()> {
//         let expected_request = to_value(json!({
//             "version": "2.0",
//             "id": 1000,
//             "method": "call",
//             "params": {
//                 "service": "common",
//                 "method": "authenticate",
//                 "args": [
//                     "my-database",
//                     "admin",
//                     "password123",
//                     {}
//                 ]
//             }
//         }))?;
//         let expected_response = to_value(json!({
//             "jsonrpc": "2.0",
//             "id": 1000,
//             "result": 1
//         }))?;

//         let request = super::authenticate("my-database", "admin", "password123", json!({}))?
//             .to_json_value()?;

//         let response = to_value(OdooApiResponse::<Authenticate>::Success(
//             JsonRpcResponseSuccess {
//                 jsonrpc: JsonRpcVersion::V2,
//                 id: 1000,
//                 result: AuthenticateResponse { uid: 1 },
//             },
//         ))?;

//         assert_eq!(request, expected_request);
//         assert_eq!(response, expected_response);

//         Ok(())
//     }

//     #[test]
//     fn version() -> Result<()> {
//         let expected_request = to_value(json!({
//             "version": "2.0",
//             "id": 1000,
//             "method": "call",
//             "params": {
//                 "service": "common",
//                 "method": "version",
//                 "args": []
//             }
//         }))?;
//         let expected_response = to_value(json!({
//             "jsonrpc": "2.0",
//             "id": 1000,
//             "result": {
//                 "server_version": "14.0+e",
//                 "server_version_info": [
//                     14,
//                     0,
//                     0,
//                     "final",
//                     0,
//                     "e"
//                 ],
//                 "server_serie": "14.0",
//                 "protocol_version": 1
//             }
//         }))?;

//         let request = super::version()?.to_json_value()?;

//         let response = to_value(OdooApiResponse::<Version>::Success(
//             JsonRpcResponseSuccess {
//                 jsonrpc: JsonRpcVersion::V2,
//                 id: 1000,
//                 result: VersionResponse {
//                     server_version: "14.0+e".into(),
//                     server_version_info: ServerVersionInfo {
//                         major: 14,
//                         minor: 0,
//                         micro: 0,
//                         release_level: "final".into(),
//                         serial: 0,
//                         enterprise: Some("e".into()),
//                     },
//                     server_serie: "14.0".into(),
//                     protocol_version: 1,
//                 },
//             },
//         ))?;

//         assert_eq!(request, expected_request);
//         assert_eq!(response, expected_response);

//         Ok(())
//     }

//     #[test]
//     fn about_basic() -> Result<()> {
//         let expected_request = to_value(json!({
//             "version": "2.0",
//             "id": 1000,
//             "method": "call",
//             "params": {
//                 "service": "common",
//                 "method": "about",
//                 "args": [false]
//             }
//         }))?;
//         let expected_response = to_value(json!({
//             "jsonrpc": "2.0",
//             "id": 1000,
//             "result": "See http://openerp.com"
//         }))?;

//         let request = super::about(false)?.to_json_value()?;

//         let response = to_value(OdooApiResponse::<About>::Success(JsonRpcResponseSuccess {
//             jsonrpc: JsonRpcVersion::V2,
//             id: 1000,
//             result: AboutResponse::Basic(AboutResponseBasic {
//                 info: "See http://openerp.com".into(),
//             }),
//         }))?;

//         assert_eq!(request, expected_request);
//         assert_eq!(response, expected_response);

//         Ok(())
//     }

//     #[test]
//     fn about_extended() -> Result<()> {
//         let expected_request = to_value(json!({
//             "version": "2.0",
//             "id": 1000,
//             "method": "call",
//             "params": {
//                 "service": "common",
//                 "method": "about",
//                 "args": [true]
//             }
//         }))?;
//         let expected_response = to_value(json!({
//             "jsonrpc": "2.0",
//             "id": 1000,
//             "result": [
//                 "See http://openerp.com",
//                 "14.0+e"
//             ]
//         }))?;

//         let request = super::about(true)?.to_json_value()?;

//         let response = to_value(OdooApiResponse::<About>::Success(JsonRpcResponseSuccess {
//             jsonrpc: JsonRpcVersion::V2,
//             id: 1000,
//             result: AboutResponse::Extended(AboutResponseExtended {
//                 info: "See http://openerp.com".into(),
//                 server_version: "14.0+e".into(),
//             }),
//         }))?;

//         assert_eq!(request, expected_request);
//         assert_eq!(response, expected_response);

//         Ok(())
//     }
// }
