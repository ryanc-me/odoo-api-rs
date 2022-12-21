//! Type-safe and full-coverage implementation of the Odoo API. Supports async, blocking, and bring-your-own-requests
//!
//! # Features
//! 1. Full coverage of the Odoo API, including the `db` service
//! 2. Support for async, blocking, and bring-your-own-requests
//! 3. Proper type implementations for endpoints (not just a `json!()` wrapper)
//!
//! # Get Started
//! First, decide whether you'll use the built-in `reqwest` async/blocking implementations,
//! or if you'll handle the requests and simply use this library for its types.
//!
//! ## Async with `reqwest`
//! Async API methods are available in the [`odoo_api::asynch`] module. Note that
//! the function arguments between async and blocking are identical.
//! 
//! ```
//! # use tokio;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // pull in API functions from the 'asynch' module
//! use odoo_api::asynch::{object, common, db};
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
//! ).await?.0;
//! # Ok(())
//! # }
//! ```
//!
//! ## Blocking with `reqwest`
//! Blocking API methods are available in the [`odoo_api::asynch`] module. Note that
//! the function arguments between async and blocking are identical.
//! 
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // pull in API functions from the 'blocking' module
//! use odoo_api::blocking::{common, db, object};
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
//! )?.0;
//! println!("Users: {:?}", resp.0)
//! # Ok(())
//! # }
//! ```
//! 
//! ## Bring your Own Requests
//! For this use-case, the Odoo API library is only used for its types.  
//! **TODO:** Update this!
//! Example:
//! ```
//! // pull in API functions from the 'types' module
//! use odoo_api::blocking::{common, db, object};
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
//! # let resp_data: request::Response = json!({
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
//! let resp: req::Response = resp_data.try_into()?;
//! let resp: odoo_api::types::ExecuteKwResponse = resp_data.try_into()?;
//! 
//! println!("Users: {:#?}", resp.data);
//! ```

pub(crate) mod jsonrpc;

pub use jsonrpc::{Error, Result, OdooApiError};

pub use jsonrpc::types;
#[cfg(feature = "async")]
pub use jsonrpc::asynch;
#[cfg(feature = "blocking")]
pub use jsonrpc::blocking;
