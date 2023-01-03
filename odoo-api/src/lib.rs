//! # odoo_api
//! The `odoo_api` crate provides a type-safe and full-coverage implementation
//! of the Odoo JSON-RPC API, including ORM and Web methods. It supports sessioning,
//! multi-database, async and blocking via [`reqwest`], and bring-your-own requests.
//!
//! ## API Methods
//!
//! For a full list of supported API methods, see [`service`].
//!
//! ## Example
//! By default, `odoo_api` uses [`reqwest`] as its HTTP implementation. It is also
//! possible to provide your own HTTP implementation (see [`OdooClient`] for more info).
//!
//! To use the default [`reqwest`] implementation, add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! odoo_api = "0.2"
//! ```
//!
//! Then make your requests:
//! ```no_run
//! use odoo_api::{OdooClient, jvec, jmap};
//!
//! # async fn test() -> odoo_api::Result<()> {
//! // build the client and authenticate
//! let url = "https://demo.odoo.com";
//! let client = OdooClient::new_reqwest_async(url)?
//!     .authenticate(
//!         "some-database",
//!         "admin",
//!         "password",
//!     ).await?;
//!
//! // fetch a list of users
//! let users = client.execute(
//!     "res.users",
//!     "search",
//!     jvec![]
//! ).send().await?;
//!
//! // fetch the login and partner_id fields from user id=1
//! let info = client.execute_kw(
//!     "res.users",
//!     "read",
//!     jvec![[1]],
//!     jmap!{
//!         "fields": ["login", "partner_id"]
//!     }
//! ).send().await?;
//!
//! // fetch a list of databases
//! let databases = client.db_list(false).send().await?;
//!
//! // fetch server version info
//! let version_info = client.common_version().send().await?;
//! # Ok(())
//! # }
//! ```

use jsonrpc::response::JsonRpcError;
use thiserror::Error;

#[macro_use]
mod macros;
pub mod client;
pub mod jsonrpc;
pub mod service;

pub use client::{AsyncClosureResult, BlockingClosureResult, OdooClient};

/// Convenience wrapper on the std `Result`
pub type Result<T> = ::std::result::Result<T, Error>;

/// An error returned by one of the Odoo API methods
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// An error during the request building phase
    #[error("Request Builder Error")]
    RequestBuilderError(String),

    /// A parsing error from the serde_json library
    ///
    /// This might be raised if the returned JSON data is invalid, or couldn't
    /// be parsed into the `XxxResponse` struct properly.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    /// An error from the [`reqwest`] library
    ///
    /// See [`reqwest::Error`] for more information.
    #[cfg(any(feature = "async", feature = "blocking"))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// The generic "Odoo Server Error"
    ///
    /// The majority of real-world errors will fall into this category. These
    /// error
    #[error("JSON-RPC Error")]
    JsonRpcError(JsonRpcError),
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        Error::RequestBuilderError(value.into())
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::RequestBuilderError(value)
    }
}

impl From<JsonRpcError> for Error {
    fn from(value: JsonRpcError) -> Self {
        Error::JsonRpcError(value)
    }
}
