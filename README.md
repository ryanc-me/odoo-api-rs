# odoo-api

[<img alt="github" src="https://img.shields.io/badge/github-ryanc--me/odoo--api--rs-master?style=flat-square&logo=github&color=4078c0" height="20">](https://github.com/ryanc-me/odoo-api-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/odoo-api?style=flat-square&logo=rust&color=2b4d28" height="20">](https://crates.io/crates/odoo-api)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/odoo-api?style=flat-square&logo=docs.rs" height="20">](https://docs.rs/odoo-api/)
[<img alt="docs.rs" src="https://img.shields.io/github/actions/workflow/status/ryanc-me/odoo-api-rs/ci.yaml?style=flat-square" height="20">](https://github.com/ryanc-me/odoo-api-rs/actions?query=branch%3Amaster)

Type-safe and full-coverage implementation of the Odoo API. Supports async, blocking, and bring-your-own-requests

## Features
- **Full Coverage** ΓÇö All JSON-RPC endpoints are covered, including the
  various database-management methods (`create_database`, `dump`, `list`, etc).

- **Flexible** ΓÇö Use the built-in async/blocking HTTP request support
  (via `reqwest`), or simply use this crate for its types and use your own
  requests library.

- **Type-Safe** ΓÇö `odoo-api` implements types for as much of the Odoo API spec
  as possible - this isn't just a `json!()` wrapper!

<br>

## Get Started
First, decide whether you want to use the built-in async/blocking support
via [`reqwest`], or if you'll bring your own requests library.

### Async with `reqwest`
```toml
## Cargo.toml
[dependencies]
odoo_api = { version = "0.1", features = ["async"] }
```

Async API methods are available in the [`odoo_api::asynch`](crate::asynch) module.
Note that the function arguments between async and blocking are identical.

```rust
// pull in API functions from the 'asynch' module
use odoo_api::asynch::{object};
use serde_json::json;

// fetch a list of all usernames
let users = object::execute_kw(
    "https://demo.odoo.com/jsonrpc",
    "my-database",
    1, "password1",
    "res.users",  "search_read",
    json!([]), // search_read doesn't take positional args
    json!({
        "domain": [[true, "=", true]],
        "fields": ["login"]
    }),
).await?.data;
```

### Blocking with `reqwest`
```toml
## Cargo.toml
[dependencies]
odoo_api = { version = "0.1", features = ["blocking"] }
```

Async API methods are available in the [`odoo_api::blocking`](crate::blocking) module.
Note that the function arguments between async and blocking are identical.

```rust
// pull in API functions from the 'blocking' module
use odoo_api::blocking::{object};
use serde_json::json;

// fetch a list of all usernames
let users = object::execute_kw(
    "https://demo.odoo.com/jsonrpc",
    "my-database",
    1, "password1",
    "res.users", "search_read",
    json!([]), // search_read doesn't take positional args
    json!({
        "domain": [[true, "=", true]],
        "fields": ["login"]
    }),
)?.data;
println!("Users: {:?}", users);
```

### Bring your Own Requests
```toml
## Cargo.toml
[dependencies]
odoo_api = { version = "0.1", features = [] }
```
Construct an object representing the request data, and use your own requests
library to perform the actual HTTP requests.

The request object is flexible and can be converted into a JSON `String`,
a [`serde_json::Value`], and also implements [`serde::Serialize`] for
libraries that accept that.

```rust
// pull in API functions from the 'types' module
use odoo_api::types::{object};
use serde_json::json;

// build the request object
let req = object::execute_kw(
    "my-database",
    1, "password1",
    "res.users",  "search_read",
    json!([]), // search_read doesn't take positional args
    json!({
        "domain": [[true, "=", true]],
        "fields": ["login"]
    }),
)?;

// convert into a JSON `String` ..
let req_data = req.to_json_string()?;
// .. or a `serde_json::Value`
let req_data = req.to_json_value()?;
// .. or, if your request library accepts types that implement [`serde::Serialize`],
// you can pass the struct directly

// fetch the response, e.g.:
// let resp_data = request.post(url).json_body(&req_data).send()?.to_json()?;

// finally, parse the response JSON using the Response objects' try_from impl
let resp: object::ExecuteKwResponse = resp_data.try_into()?;

println!("Users: {:#?}", resp.data);
```

<br>

## Optional Features
* **async** - Enable async HTTP request support via [`reqwest`]
* **blocking** - Enable blocking HTTP request support via [`reqwest`]
