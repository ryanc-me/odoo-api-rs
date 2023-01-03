//! The Odoo "Web" pseudo-service
//!
//! This isn't actually a service, but a set of JSON-RPC compatible endpoints
//! that Odoo exposes. Generally these are used by the webclient, and offer
//! functionality that can be achieved with `execute` and `execute_kw`

use crate as odoo_api;
use crate::jsonrpc::OdooWebMethod;
use odoo_api_macros::odoo_web;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Docs TBC
#[odoo_web(path = "/web/session/authenticate", name = "web_session_authenticate")]
#[derive(Debug, Serialize)]
pub struct SessionAuthenticate {
    pub(crate) db: String,
    pub(crate) login: String,
    pub(crate) password: String,
}

/// Docs TBC
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SessionAuthenticateResponse {
    pub data: Value,
}
