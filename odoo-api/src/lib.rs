//! Type-safe and full-coverage implementation of the Odoo API. Supports async, blocking, and bring-your-own-requests
//!
//! # Features
//! 1. Full coverage of the Odoo API, including the `db` service
//! 2. Support for async, blocking, and bring-your-own-requests
//! 3. Proper type implementations for endpoints (not just a `json!()` wrapper)
//!
//! # Get Started
//! First, decide whether you want to use the built-in async/blocking support
//! via [`reqwest`], or if you'll bring your own requests library.
//!
//! ## Async with `reqwest`
//! ```toml
//! ## Cargo.toml
//! [dependencies]
//! odoo_api = { version = "0.1", features = ["async"] }
//! ```
//! 
//! Async API methods are available in the [`odoo_api::asynch`](crate::asynch) module.
//! Note that the function arguments between async and blocking are identical.
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
//!     "res.users",  "search_read",
//!     json!([]), // search_read doesn't take positional args
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
//! ```toml
//! ## Cargo.toml
//! [dependencies]
//! odoo_api = { version = "0.1", features = ["blocking"] }
//! ```
//! 
//! Async API methods are available in the [`odoo_api::blocking`](crate::blocking) module.
//! Note that the function arguments between async and blocking are identical.
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
//!     json!([]), // search_read doesn't take positional args
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
//! ```toml
//! ## Cargo.toml
//! [dependencies]
//! odoo_api = { version = "0.1", features = [] }
//! ```
//! Construct an object representing the request data, and use your own requests
//! library to perform the actual HTTP requests.
//! 
//! The request object is flexible and can be converted into a JSON `String`,
//! a [`serde_json::Value`], and also implements [`serde::Serialize`] for
//! libraries that accept that.
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
//!     "res.users",  "search_read",
//!     json!([]), // search_read doesn't take positional args
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
//! # Optional Features
//! * **async** - Enable async HTTP request support via [`reqwest`]
//! * **blocking** - Enable blocking HTTP request support via [`reqwest`]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

pub mod jsonrpc;

pub use jsonrpc::types;
#[cfg(feature = "async")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async")))]
pub use jsonrpc::asynch;
#[cfg(feature = "blocking")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "blocking")))]
pub use jsonrpc::blocking;
