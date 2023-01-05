//! The user-facing API types
//!
//! This module provides a user-facing API for Odoo JSON-RPC methods.

pub use http_impl::closure_async::ClosureResult as AsyncClosureResult;
pub use http_impl::closure_blocking::ClosureResult as BlockingClosureResult;
pub use odoo_client::{AuthState, Authed, NotAuthed, OdooClient, RequestImpl};
pub use odoo_request::OdooRequest;

pub use http_impl::closure_async::ClosureAsync;
pub use http_impl::closure_blocking::ClosureBlocking;
pub use http_impl::reqwest_async::ReqwestAsync;
pub use http_impl::reqwest_blocking::ReqwestBlocking;

mod http_impl;
mod odoo_client;
mod odoo_request;
