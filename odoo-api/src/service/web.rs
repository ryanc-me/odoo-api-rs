//! The Odoo "Web" pseudo-service
//!
//! This isn't actually a service, but a set of JSON-RPC compatible endpoints
//! that Odoo exposes. Generally these are used by the webclient, and offer
//! functionality that can be achieved with `execute` and `execute_kw`

use crate as odoo_api;
use crate::jsonrpc::OdooWebMethod;
use odoo_api_macros::odoo_web;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};
use serde_json::Value;

//TODO: /web/session/get_lang_list (only v15+?)
//TODO: /web/session/check
//TODO: /web/session/change_password
//TODO: /web/session/get_session_info
//TODO: /web/session/modules
//TODO: /web/session/modules
//TODO: /web/session/destroy
//TODO: /web/session/logout
//TODO: /web/dataset/resequence
//TODO: /web/dataset/call
//TODO: /web/dataset/call_kw
//TODO: /web/dataset/load
//TODO: /web/dataset/search_read

/// Authenticate to an Odoo database
///
/// This method performs a bona-fide Odoo "authentication"; it checks the user
/// name/password, and creates a new `session_id` (which is returned via the
/// `Set-Cookie` header).
///
/// Note that by itself, this function isn't able to parse the `session_id` token,
/// so it probably isn't very useful.
///
/// See [`authenticate`](crate::client::OdooClient::authenticate) if you'd like to
/// authenticate an `OdooClient`.
///
/// Reference: [web/controllers/session.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/addons/web/controllers/session.py#L29-L43)
#[odoo_web(
    path = "/web/session/authenticate",
    name = "web_session_authenticate",
    auth = false
)]
#[derive(Debug, Serialize)]
pub struct SessionAuthenticate {
    pub(crate) db: String,
    pub(crate) login: String,
    pub(crate) password: String,
}

/// Represents the response to an Odoo [`SessionAuthenticate`] call
///
/// Note that the generated `session_id` is not returned here. The response
/// data contains some information about the Odoo session.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionAuthenticateResponse {
    pub data: Value,
}

/// List the available databases
///
/// This function *doesn't require a session token*, so it can be run on an OdooClient
/// that hasn't been authenticated yet.
///
/// Reference: [web/controller/database.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/addons/web/controllers/database.py#L176-L183)
#[odoo_web(path = "/web/database/list", name = "web_database_list", auth = false)]
#[derive(Debug)]
pub struct DatabaseList {}

// DatabaseList has no fields, but needs to output in JSON: `[]`
impl Serialize for DatabaseList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let state = serializer.serialize_tuple(0)?;
        state.end()
    }
}

/// Represents the response to an Odoo [`DatabaseList`] call
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DatabaseListResponse {
    pub databases: Vec<String>,
}
