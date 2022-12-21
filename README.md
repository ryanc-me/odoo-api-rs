# odoo-api-rs

Type-safe and full-coverage implementation of the Odoo API. Supports async, blocking, and bring-your-own-requests

## Features
 1. Full coverage of the Odoo API, including the `db` service
 2. Support for async, blocking, and bring-your-own-requests
 3. Proper type implementations for endpoints (not just a `json!()` wrapper)

## Getting Started
First, decide whether you'll use the built-in `reqwest` async/blocking implementations,
or if you'll handle the requests and simply use this library for its types.

### Reqwest (Async)

`cargo.toml`:
```
[features]
odoo_api = { version = "0.1", features = ["reqwest"] }
```

Example:
```rust
use odoo_api::reqwest::{common, object};
use odoo_api::OdooApiResponse
use serde_json::{Map, Value};

async fn test() {
    // fetch the Odoo server version
    let version_resp = common::version().await?;
    match version_resp {
        OdooApiResponse::Success({ result, .. }) => {
            // use the data
            println!("Odoo Version: {:?}", result.server_version);
        }
        OdooApiResponse::Success({ error, ... }) => {
            // an error occured
            panic!("{:?}", error);
        }
    };

    // read some fields from users 1 and 2
    let mut args = Vec::<Value>::new();
    let mut kwargs = Map::<String, Value>::new();
    args.push(json!([1, 2]));
    kwargs.insert(
        "fields".into(),
        json!(["id", "login", "name"])
    );
    let execute_resp = object::execute_kw(
        "my-database",
        2, // uid
        "password123",
        "res.users",
        "read",
        args,
        kwargs
    ).await?;

    match execute_resp {
        OdooApiResponse::Success({ result, .. }) => {
            // use the data
            println!("Data: {:?}", result.to_string_pretty());
        }
        OdooApiResponse::Success({ error, ... }) => {
            // an error occured
            panic!("{:?}", error);
        }
    }
}
```
// build the API method struct, and convert to JSON string
let request = common::about(true).to_json_string();

// perform the request and convert response to JSON string
// let resp_json = ...
# let resp_json = "{\"jsonrpc\":\"2.0\",\"id\":100,\"result\":\"See http://openerp.com\"}";

// parse the JSON string into a response object
let response = request.parse_json_response(resp_json);
let response = response.expect("Failed to parse JSON data");

match response {
    OdooApiResponse::Success(resp) => {
        // do something with the data
        println!("Got data: {:?}", resp.result);
    },
    OdooApiResponse::Error(resp) => {
        // do something with the error
        println!("Request failed! {:?}", resp.error);
    }
}
```

### Bring your Own Requests
For this use-case, the Odoo API library is only used for its types.  
Example:
```rust
use odoo_api::types::{common};
use odoo_api::OdooApiResponse;

// build the API method struct, and convert to JSON string
let request = common::about(true).to_json_string();

// perform the request and convert response to JSON string
// let resp_json = ...
# let resp_json = "{\"jsonrpc\":\"2.0\",\"id\":100,\"result\":\"See http://openerp.com\"}";

// parse the JSON string into a response object
let response = request.parse_json_response(resp_json);
let response = response.expect("Failed to parse JSON data");

match response {
    OdooApiResponse::Success(resp) => {
        // do something with the data
        println!("Got data: {:?}", resp.result);
    },
    OdooApiResponse::Error(resp) => {
        // do something with the error
        println!("Request failed! {:?}", resp.error);
    }
}
```