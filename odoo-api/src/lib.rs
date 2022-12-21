//! # odoo-api
//! Type-safe and full-coverage implementation of the Odoo API. Supports async, blocking, and bring-your-own-requests
//!
//! ## Features
//! 1. Full coverage of the Odoo API, including the `db` service
//! 2. Support for async, blocking, and bring-your-own-requests
//! 3. Proper type implementations for endpoints (not just a `json!()` wrapper)
//!
//! ## Get Started
//! First, decide whether you'll use the built-in `reqwest` async/blocking implementations,
//! or if you'll handle the requests and simply use this library for its types.
//!
//! ### Bring your Own Requests
//! For this use-case, the Odoo API library is only used for its types.  
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

#[cfg(test)]
mod test {
    use tokio;
    use crate::reqwest::{common, object};
    use super::OdooApiResponse;
    use serde_json::{Map, Value, json};

    #[test]
    fn test() {
        use super::jsonrpc;
        // test = jsonrpc::
    }

    // #[tokio::test]
    // async fn test() {
    //     // fetch the Odoo server version
    //     // let version_resp = common::version().await?;
    //     // match version_resp {
    //     //     OdooApiResponse::Success({ result, .. }) => {
    //     //         // use the data
    //     //         println!("Odoo Version: {:?}", result.server_version);
    //     //     }
    //     //     OdooApiResponse::Success({ error, ... }) => {
    //     //         // an error occured
    //     //         panic!("{:?}", error);
    //     //     }
    //     // };

    //     // read some fields from users 1 and 2
    //     let mut args = Vec::<Value>::new();
    //     let mut kwargs = Map::<String, Value>::new();
    //     args.push(json!([1, 2, 3, 4, 5]));
    //     kwargs.insert(
    //         "fields".into(),
    //         json!(["id", "login", "name"])
    //     );
    //     let execute_resp = object::execute_kw(
    //         "https://demo.odoo.com/jsonrpc",
    //         "my-database",
    //         2, // uid
    //         "password123",
    //         "res.users",
    //         "read",
    //         args,
    //         kwargs
    //     ).await.unwrap();

    //     match execute_resp {
    //         OdooApiResponse::Success(data) => {
    //             // use the data
    //             println!("Data: {:?}", data.result);
    //         }
    //         OdooApiResponse::Error(data) => {
    //             // an error occured
    //             panic!("{:?}", data.error);
    //         }
    //     }
    // }
}