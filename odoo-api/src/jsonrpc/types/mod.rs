//! Types describing the Odoo API, along with functions to conveniently build those types

pub mod db;
pub mod object;
pub mod common;

pub(crate) use super::{OdooID, OdooApiMethod, OdooApiRequest, OdooApiResponse, JsonRpcVersion, JsonRpcMethod, JsonRpcRequestParams};
