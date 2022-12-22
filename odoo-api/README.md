# odoo-api

[<img alt="github" src="https://img.shields.io/badge/github-ryanc--me/odoo--api--rs-master?style=flat-square&logo=github&color=4078c0" height="20">](https://github.com/ryanc-me/odoo-api-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/odoo-api?style=flat-square&logo=rust&color=f9f7ec" height="20">](https://crates.io/crates/odoo-api)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/odoo-api?style=flat-square&logo=docs.rs" height="20">](https://docs.rs/odoo-api/)
[<img alt="docs.rs" src="https://img.shields.io/github/actions/workflow/status/ryanc-me/odoo-api-rs/ci.yaml?style=flat-square" height="20">](https://github.com/ryanc-me/odoo-api-rs/actions?query=branch%3Amaster)

Type-safe and full-coverage implementation of the Odoo API. Supports async, blocking, and bring-your-own-requests

## Features
- **Full Coverage** - All JSON-RPC endpoints are covered, including the
  various database-management methods (`create_database`, `dump`, `list`, etc).
  Support for some common ORM methods is also included (`read`, `search_read`, `create`, etc).

- **Flexible** - Use the built-in async/blocking HTTP request support
  (via `reqwest`), or simply use this crate for its types and use your own
  requests library. The API request and response types all implement `Serialize`,
  functions to convert into `serde_json::Value`, and functions to dump the
  request out as a plain JSON `String`, so almost any requests library will work.

- **Type-Safe** - The `odoo-api` crate implements types for as much of the
  Odoo API as possible, right up to the positional & keyword arguments for
  some ORM methods.

<br>

## Get Started
First, decide how you want to use this library:
- Using the built-in [async](#async-with-reqwest) support via `reqwest`
- Using the built-in [blocking](#blocking-with-reqwest) support via `reqwest`
- Use this library for its types only, and [bring your own requests library](#bring-your-own-requests)

### Async with `reqwest`

[**Documentation**](https://docs.rs/odoo-api/latest/odoo_api/jsonrpc/asynch/index.html)

```toml
## Cargo.toml
[dependencies]
odoo_api = { version = "0.1", features = ["async"] }
```

```rust
// pull in API functions from the 'asynch' module
use odoo_api::asynch::{object};
use serde_json::json;

// fetch a list of all usernames
let users = object::execute_kw(
    "https://demo.odoo.com/jsonrpc",
    "my-database",
    1, "password1",
    "res.users", "search_read",
    json!([]),
    json!({
        "domain": [[true, "=", true]],
        "fields": ["login"]
    }),
).await?.data;
```

### Blocking with `reqwest`

[**Documentation**](https://docs.rs/odoo-api/latest/odoo_api/jsonrpc/blocking/index.html)

```toml
## Cargo.toml
[dependencies]
odoo_api = { version = "0.1", features = ["blocking"] }
```

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
    json!([]),
    json!({
        "domain": [[true, "=", true]],
        "fields": ["login"]
    }),
)?.data;
println!("Users: {:?}", users);
```

### Bring your Own Requests

See the link below for more info on building the request types, converting
to JSON `String` or `serde_json::Value`, and parsing the response.

[**Documentation**](https://docs.rs/odoo-api/latest/odoo_api/jsonrpc/types/index.html)

```toml
## Cargo.toml
[dependencies]
odoo_api = { version = "0.1", features = [] }
```

```rust
// pull in API functions from the 'types' module
use odoo_api::types::{object};
use serde_json::json;

// build the request object
let req = object::execute_kw(
    "my-database",
    1, "password1",
    "res.users", "search_read",
    json!([]),
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

