//! The user-facing API types
//!
//! This module provides a user-facing API for Odoo JSON-RPC methods.
//!
//! ##

pub use odoo_client::{AuthState, Authed, NotAuthed, OdooClient, RequestImpl};
pub use odoo_request::OdooRequest;
pub use req_impl::closure_async::ClosureResult as AsyncClosureResult;
pub use req_impl::closure_blocking::ClosureResult as BlockingClosureResult;

pub use req_impl::closure_async::ClosureAsync;
pub use req_impl::closure_blocking::ClosureBlocking;
pub use req_impl::reqwest_async::ReqwestAsync;
pub use req_impl::reqwest_blocking::ReqwestBlocking;

mod odoo_client;
mod odoo_request;
mod req_impl;
