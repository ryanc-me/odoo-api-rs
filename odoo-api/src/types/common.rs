use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};
use odoo_api_macros::odoo_api_request;
use crate::{OdooID};


/// Represents a **`common/login`** API call.
///
/// **Service**: `common`  
/// **Method**: `login`  
/// **Request**: [`Login`]  
/// **Response**: [`LoginResponse`]  
///
/// This method performs a "login" to the Odoo server, and returns the corresponding
/// user ID (`uid`).
///
/// Note that the Odoo API is stateless, so this function doesn't return a long-lived
/// token - it only returns the `uid`. Typically this method is used to determine
/// if the user exists in the Odoo database, and to find their corresponding `uid`.
/// Once fetched, the `uid` can be saved for future calls.
///
/// Example:
/// ```rust
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
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("common", "login")]
pub struct Login {
    /// The database name
    pub db: String,

    /// The username (e.g., email)
    pub login: String,

    /// The user password
    pub password: String,
}

/// Represents the response to an Odoo [`Login`] call.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct LoginResponse {
    // #[serde(deserialize_with = "")]
    pub uid: OdooID
}

/// Represents a **`common/authenticate`** API call.
///
/// **Service**: `common`  
/// **Method**: `authenticate`  
/// **Request**: [`Authenticate`]  
/// **Response**: [`AuthenticateResponse`]  
///
/// This method performs a "login" to the Odoo server, and returns the corresponding
/// user ID (`uid`). It is identical to [`login`], except that it accepts an extra
/// param `user_agent_env`, which is normally sent by the browser.
///
/// This method is inteded for browser-based API implementations. You should use [`Login`] or [`login`] instead.
///
/// Example:
/// ```rust
/// use serde_json::{json, Map, Value};
/// use odoo_api::types::common;
///
/// let mut env = Map::<String, Value>::new();
/// env.insert("base_location".into(), json!("www.example.com"));
///
/// let request = common::authenticate(
///     "my-database",
///     "user@example.com",
///     "password1",
///     env
/// );
/// ```
///
/// See: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L22-L29)
/// See also: [base/models/res_users.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/addons/base/models/res_users.py#L762-L787)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("common", "authenticate")]
pub struct Authenticate {
    /// The database name
    pub db: String,

    /// The username (e.g., email)
    pub login: String,

    /// The user password
    pub password: String,

    /// A mapping of user agent env entries
    pub user_agent_env: Map<String, Value>
}

/// Represents the response to an Odoo [`Authenticate`] call.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AuthenticateResponse {
    pub uid: OdooID
}


/// Represents a **`common/version`** API call.
///
/// **Service**: `common`  
/// **Method**: `version`  
/// **Request**: [`Version`]  
/// **Response**: [`VersionResponse`]  
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
/// ```rust
/// use odoo_api::types::common;
///
/// let request = common::version();
/// ```
///
/// See: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L31-L32)
/// See also: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L12-L17)
/// See also: [odoo/release.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/release.py)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("common", "version")]
pub struct Version {}

/// Represents the response to an Odoo [`Version`] call.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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


/// Represents a **`common/about`** API call.
///
/// **Service**: `common`  
/// **Method**: `about`  
/// **Request**: [`About`]  
/// **Response**: [`AboutResponse`]  
///
/// Returns a link to the old OpenERP website, and optionally the "basic" Odoo
/// version string (e.g. `16.0+e`).
///
/// This call isn't particularly useful on its own - you probably want to use [`Version`]
/// or [`version`] instead.
///
/// Example:
/// ```rust
/// use odoo_api::types::common;
///
/// let request = common::about(true);
/// ```
///
/// See: [odoo/service/common.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/common.py#L34-L45)  
/// See also: [odoo/release.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/release.py)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("common", "about")]
pub struct About {
    pub extended: bool,
}

//TODO: flat deserializ so we can have either `result: "http://..."` or `result: ["http://..", "14.0+e"]`
/// Represents the response to an Odoo [`About`] call.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum AboutResponse {
    /// Basic response; includes only the `info` string
    Basic(AboutResponseBasic),

    /// Extended response; includes `info` string and version info
    Extended(AboutResponseExtended)
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct AboutResponseBasic {
    /// The "info" string
    /// 
    /// At the time of writing, this is hard-coded to `See http://openerp.com`
    info: String
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct AboutResponseExtended {
    /// The "info" string
    /// 
    /// At the time of writing, this is hard-coded to `See http://openerp.com`
    info: String,

    /// The "pretty" version, normally something like `16.0+e` or `15.0`
    ///
    /// Note that this is only returned when the original reques was made with
    /// `extended: true` (see [`AboutResponse`])
    pub server_version: String,
}

#[cfg(test)]
mod test {
    use serde_json::{json, from_value};
    use super::*;
    use crate::{JsonRpcVersion};
    use crate::request::{OdooApiRequest};
    use crate::response::{OdooApiResponse, JsonRpcResponseSuccess};

    #[test]
    fn login() {
        let mock_request: OdooApiRequest<Login> = from_value(json!({
            "version": "2.0",
            "id": 1000,
            "method": "call",
            "params": {
                "service": "common",
                "method": "login",
                "args": [
                    "test-database",
                    "user@example.com",
                    "password123"
                ]
            }
        })).expect("Failed to parse mock request into struct");
        let mock_response: OdooApiResponse<Login> = from_value(json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": 2
        })).expect("Failed to parse mock response into struct");

        let request = super::login(
            "test-database",
            "user@example.com",
            "password123",
        );
        let response = OdooApiResponse::Success(
            JsonRpcResponseSuccess::<Login> {
                jsonrpc: JsonRpcVersion::V2,
                id: 1000,
                result: LoginResponse {
                    uid: 2
                }
            }
        );

        assert_eq!(request, mock_request);
        assert_eq!(response, mock_response);

        println!("Mock Request: {}", mock_request.to_json_string().unwrap());
        println!("Real Request: {}", request.to_json_string().unwrap());
        println!("Mock Response: {}", mock_response.to_json_string().unwrap());
        println!("Real Response: {}", response.to_json_string().unwrap());

    }

    #[test]
    fn authenticate() {
        let _mock_request = "{
            \"version\": \"2.0\",
            \"id\": 1000,
            \"method\": \"call\",
            \"params\": {
                \"service\": \"common\",
                \"method\": \"authenticate\",
                \"args\": [
                    \"test-database\",
                    \"user@example.com\",
                    \"password123\",
                    {}
                ]
            }
        }";
        let _mock_response = "{
            \"jsonrpc\": \"2.0\",
            \"id\": 1000,
            \"result\": 2
        }";

    }

    #[test]
    fn version() {

    }

    #[test]
    fn about() {

    }
}