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
//! ```
//! # use tokio;
//! # #[tokio::main]
//! # async fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // pull in the async methods
//! use odoo_api::nonblocking::{common, db, object};
//! use serde_json::json;
//!
//! 
//! // fetch a list of databases
//! let databases = db::list_lang("https://demo.odoo.com/jsonrpc").await?.databases;
//! println!("Databases {:?}", databases);
//!
//! // perform authentication
//! let uid = common::login(
//!     "https://demo.odoo.com/jsonrpc",
//!     "my-database",
//!     "admin", // user
//!     "admin", // pass
//! ).await?.uid;
//!
//! // fetch a list of all usernames
//! let users = object::execute(
//!     "https://demo.odoo.com/jsonrpc",
//!     "my-database",
//!     uid,
//!     "admin",
//!     "res.users",
//!     "search_read",
//!     json!([]),
//!     json!({
//!         "domain": [[true, "=", true]],
//!         "fields": ["login"]
//!     }),
//! ).await?.0
//! println!("Users: {:?}", users);
//! # Ok(())
//! # }
//! ```
//!
//! ## Blocking with `reqwest`
//! ```
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // pull in the blocking methods
//! use odoo_api::blocking::{common, db, object};
//! use serde_json::json;
//!
//!
//! // fetch a list of databases
//! let databases = db::list_lang("https://demo.odoo.com/jsonrpc")?.databases;
//! println!("Databases {:?}", databases);
//!
//! // perform authentication
//! let uid = common::login(
//!     "https://demo.odoo.com/jsonrpc",
//!     "my-database",
//!     "admin", // user
//!     "admin", // pass
//! )?.uid;
//!
//! // fetch a list of all usernames
//! let users = object::execute(
//!     "https://demo.odoo.com/jsonrpc",
//!     "my-database",
//!     uid,
//!     "admin",
//!     "res.users",
//!     "search_read",
//!     json!([]),
//!     json!({
//!         "domain": [[true, "=", true]],
//!         "fields": ["login"]
//!     }),
//! )?.users;
//! println!("Users: {:?}", resp.0)
//! # Ok(())
//! # }
//! ```
//! 
//! ## Bring your Own Requests
//! For this use-case, the Odoo API library is only used for its types.  
//! **TODO:** Update this!
//! Example:
//! ```text
//! use odoo_api::types::{common};
//! use odoo_api::OdooApiResponse;
//!
//! // build the API method struct, and convert to JSON string
//! let request = common::about(true).to_json_string();
//!
//! // perform the request and convert response to JSON string
//! // let resp_json = ...
//! # let resp_json = "{\"jsonrpc\":\"2.0\",\"id\":100,\"result\":\"See http://openerp.com\"}";
//!
//! // parse the JSON string into a response object
//! let response = request.parse_json_response(resp_json);
//! let response = response.expect("Failed to parse JSON data");
//! 
//! match response {
//!     OdooApiResponse::Success(resp) => {
//!         // do something with the data
//!         println!("Got data: {:?}", resp.result);
//!     },
//!     OdooApiResponse::Error(resp) => {
//!         // do something with the error
//!         println!("Request failed! {:?}", resp.error);
//!     }
//! }
//! ```

pub(crate) mod jsonrpc;

pub use jsonrpc::{Error, Result, OdooApiError};

pub use jsonrpc::types;
#[cfg(feature = "nonblocking")]
pub use jsonrpc::nonblocking;
#[cfg(feature = "blocking")]
pub use jsonrpc::blocking;
