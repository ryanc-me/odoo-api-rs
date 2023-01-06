//! # odoo_api
//! The `odoo_api` crate provides a type-safe and full-coverage implementation
//! of the Odoo JSON-RPC API, including ORM and Web methods. It supports sessioning,
//! multi-database, async and blocking via [`reqwest`], and bring-your-own requests.
//!
//! ## API Methods
//!
//! For a full list of supported API methods, see [`service`].
//!
//! ## Bring your own requests
//!
//! By default, `odoo_api` uses [`reqwest`] as its HTTP implementation. It is also
//! possible to provide your own HTTP implementation (see [`OdooClient`] for more info).
//!
//! ## Example
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
//! # #[cfg(not(feature = "types-only"))]
//! use odoo_api::{OdooClient, jvec, jmap};
//!
//! # #[cfg(not(feature = "types-only"))]
//! # async fn test() -> odoo_api::client::Result<()> {
//! // build the client and authenticate
//! let url = "https://demo.odoo.com";
//! let mut client = OdooClient::new_reqwest_async(url)?
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

// The `types-only` feature implies that the `client` module isn't included, so
// `async` and `blocking` have no effect
#[cfg(all(feature = "types-only", any(feature = "async", feature = "blocking")))]
std::compile_error!(
    "The `types-only` feature is mutually exclusive with the `async` and `blocking` \
     features. Please disable the `async` and `blocking` by adding `default-features = false` \
     to your Cargo.toml"
);

pub mod service;

#[macro_use]
mod macros;

#[cfg(not(feature = "types-only"))]
pub mod client;

#[cfg(not(feature = "types-only"))]
pub use client::{AsyncClosureResult, BlockingClosureResult, OdooClient};

pub mod jsonrpc;
pub use jsonrpc::OdooId;
