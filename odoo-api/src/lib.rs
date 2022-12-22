//! Type-safe and full-coverage implementation of the Odoo API. Supports async, blocking, and bring-your-own-requests
//!
//! # Features
//! - **Full Coverage** - All JSON-RPC endpoints are covered, including the
//!   various database-management methods (`create_database`, `dump`, `list`, etc).
//!   Support for some common ORM methods is also included (`read`, `search_read`, `create`, etc).
//!
//! - **Flexible** - Use the built-in async/blocking HTTP request support
//!   (via `reqwest`), or simply use this crate for its types and use your own
//!   requests library. The API request and response types all implement `Serialize`,
//!   functions to convert into `serde_json::Value`, and functions to dump the
//!   request out as a plain JSON `String`, so almost any requests library will work.
//!
//! - **Type-Safe** - The `odoo-api` crate implements types for as much of the
//!   Odoo API as possible, right up to the positional & keyword arguments for
//!   some ORM methods.
//!
//! <br>
//!
//! # Get Started
//! First, decide how you want to use this library:
//! - Using the built-in [async](#async-with-reqwest) support via `reqwest`
//! - Using the built-in [blocking](#blocking-with-reqwest) support via `reqwest`
//! - Use this library for its types only, and [bring your own requests library](#bring-your-own-requests)
//!
//! ## Async with `reqwest`
//!
//! [**Documentation**](https://docs.rs/odoo-api/latest/odoo_api/jsonrpc/asynch/index.html)
//!
//! ```toml
//! ## Cargo.toml
//! [dependencies]
//! odoo_api = { version = "0.1", features = ["async"] }
//! ```
//!
//! ```no_run
//! # use tokio;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // pull in API functions from the 'asynch' module
//! use odoo_api::asynch::{object};
//! use serde_json::json;
//!
//! // fetch a list of all usernames
//! let users = object::execute_kw(
//!     "https://demo.odoo.com/jsonrpc",
//!     "my-database",
//!     1, "password1",
//!     "res.users", "search_read",
//!     json!([]),
//!     json!({
//!         "domain": [[true, "=", true]],
//!         "fields": ["login"]
//!     }),
//! ).await?.data;
//! # Ok(())
//! # }
//! ```
//!
//! ## Blocking with `reqwest`
//!
//! [**Documentation**](https://docs.rs/odoo-api/latest/odoo_api/jsonrpc/blocking/index.html)
//!
//! ```toml
//! ## Cargo.toml
//! [dependencies]
//! odoo_api = { version = "0.1", features = ["blocking"] }
//! ```
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // pull in API functions from the 'blocking' module
//! use odoo_api::blocking::{object};
//! use serde_json::json;
//!
//! // fetch a list of all usernames
//! let users = object::execute_kw(
//!     "https://demo.odoo.com/jsonrpc",
//!     "my-database",
//!     1, "password1",
//!     "res.users", "search_read",
//!     json!([]),
//!     json!({
//!         "domain": [[true, "=", true]],
//!         "fields": ["login"]
//!     }),
//! )?.data;
//! println!("Users: {:?}", users);
//! # Ok(())
//! # }
//! ```
//!
//! ## Bring your Own Requests
//!
//! See the link below for more info on building the request types, converting
//! to JSON `String` or `serde_json::Value`, and parsing the response.
//!
//! [**Documentation**](https://docs.rs/odoo-api/latest/odoo_api/jsonrpc/types/index.html)
//!
//! ```toml
//! ## Cargo.toml
//! [dependencies]
//! odoo_api = { version = "0.1", features = [] }
//! ```
//!
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // pull in API functions from the 'types' module
//! use odoo_api::types::{object};
//! use serde_json::json;
//!
//! // build the request object
//! let req = object::execute_kw(
//!     "my-database",
//!     1, "password1",
//!     "res.users", "search_read",
//!     json!([]),
//!     json!({
//!         "domain": [[true, "=", true]],
//!         "fields": ["login"]
//!     }),
//! )?;
//!
//! // convert into a JSON `String` ..
//! let req_data = req.to_json_string()?;
//! // .. or a `serde_json::Value`
//! let req_data = req.to_json_value()?;
//! // .. or, if your request library accepts types that implement [`serde::Serialize`],
//! // you can pass the struct directly
//!
//! // fetch the response, e.g.:
//! // let resp_data = request.post(url).json_body(&req_data).send()?.to_json()?;
//! # let resp_data = json!({
//! #     "jsonrpc": "2.0",
//! #     "id": 1000,
//! #     "result": [
//! #         {"id": 2, "login": "admin"},
//! #         {"id": 7, "login": "portal"},
//! #         {"id": 6, "login": "demo"}
//! #     ]
//! # });
//!
//! // finally, parse the response JSON using the Response objects' try_from impl
//! let resp: object::ExecuteKwResponse = resp_data.try_into()?;
//!
//! println!("Users: {:#?}", resp.data);
//! # Ok(())
//! # }
//! ```
//!
//!<br>
//!
//! # Optional Features
//! * **async** - Enable async HTTP request support via [`reqwest`]
//! * **blocking** - Enable blocking HTTP request support via [`reqwest`]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod jsonrpc;

#[cfg(feature = "async")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async")))]
pub use jsonrpc::asynch;
#[cfg(feature = "blocking")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "blocking")))]
pub use jsonrpc::blocking;
pub use jsonrpc::types;
