//! The `odoo-api` odoo-api is a Rust library crate that provides a user-friendly interface
//! to interact with the Odoo JSONRPC and ORM APIs, while preserving strong typing. It
//! includes both async and blocking support out of the box, and allows users to provide
//! their own implementations if needed.
//!
//! See the [Example](#example) section below for a brief example, or the [`client`] module for more in-depth examples.
//!
//! ## Features
//!
//!  - **Strong typing**: `odoo-api` prioritizes the use of concrete types wherever
//!     possible, rather than relying on generic `json!{}` calls.
//!  - **Async and blocking support**: the library provides both async and blocking
//!     HTTP impls via [`reqwest`], and allows users to easily provide their own HTTP
//!     impl via a shim closure.
//!  - **JSONRPC API support**: including database management (create, duplicate, etc),
//!     translations, and generic `execute` and `execute_kw`
//!  - **ORM API support**: including user-friendly APIs for the CRUD, `search_read`,
//!     security rule checking, and more
//!  - **Types-only**: allowing you to include this library for its types only. See
//!     [Types Only](#types-only) below for more info
//!
//! ### Supported API Methods
//!
//! See the [`service`] module for a full list of supported API methods.
//!
//! ### Bring Your Own Requests
//!
//! Do you already have an HTTP library in your dependencies (e.g., `reqwest`)?
//!
//! The `odoo-api` crate allows you to use your existing HTTP library by writing a
//! simple shim closure. See [`client::ClosureAsync`] or [`client::ClosureBlocking`]
//! for more info.
//!
//! ### Types Only
//!
//! The crate offers a `types-only` feature. When enabled, the library only exposes
//! the API request & response types, along with `Serialize` and `Deserialize` impls.
//! The async/blocking impls (and the [`reqwest`] dependency) are dropped when this
//! feature is active.
//!
//! See the [`jsonrpc`] module for information on `types-only`.
//!
//! ## Example
//!
//! <br />
//! Add the following to your `Cargo.toml`:
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
//! // build the client
//! let url = "https://odoo.example.com";
//! let mut client = OdooClient::new_reqwest_async(url)?;
//!
//! // authenticate with `some-database`
//! let mut client = client.authenticate(
//!     "some-database",
//!     "admin",
//!     "password",
//! ).await?;
//!
//! // fetch a list of users with the `execute` method
//! let users = client.execute(
//!     "res.users",
//!     "search",
//!     jvec![
//!         [["active", "=", true], ["login", "!=", "__system__"]]
//!     ]
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
//! // create 2 new partners with the `create` ORM method
//! let partners = client.create(
//!     "res.partner",
//!     jvec![{
//!         "name": "Alice",
//!         "email": "alice@example.com",
//!         "phone": "555-555-5555",
//!     }, {
//!         "name": "Bob",
//!         "email": "bob@example.com",
//!         "phone": "555-555-5555",
//!     }]
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
pub use client::{AsyncClosureReturn, BlockingClosureReturn, OdooClient};

pub mod jsonrpc;
pub use jsonrpc::OdooId;
