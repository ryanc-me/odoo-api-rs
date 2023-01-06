//! The user-facing API types
//!
//! This module provides a user-facing API for Odoo JSON-RPC methods.

pub use http_impl::closure_async::ClosureReturn as AsyncClosureReturn;
pub use http_impl::closure_blocking::ClosureReturn as BlockingClosureReturn;
pub use odoo_client::{AuthState, Authed, NotAuthed, OdooClient, RequestImpl};
pub use odoo_request::OdooRequest;

pub use error::{Error, Result};
pub use http_impl::closure_async::ClosureAsync;
pub use http_impl::closure_blocking::ClosureBlocking;

#[cfg(feature = "async")]
pub use http_impl::reqwest_async::ReqwestAsync;

#[cfg(feature = "blocking")]
pub use http_impl::reqwest_blocking::ReqwestBlocking;

pub mod error;
mod http_impl;
mod odoo_client;
mod odoo_request;
