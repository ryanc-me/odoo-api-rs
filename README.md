# odoo-api

[<img alt="github" src="https://img.shields.io/badge/github-ryanc--me/odoo--api--rs-master?style=flat-square&logo=github&color=4078c0" height="20">](https://github.com/ryanc-me/odoo-api-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/odoo-api?style=flat-square&logo=rust&color=f9f7ec" height="20">](https://crates.io/crates/odoo-api)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/odoo-api?style=flat-square&logo=docs.rs" height="20">](https://docs.rs/odoo-api/)
[<img alt="docs.rs" src="https://img.shields.io/github/actions/workflow/status/ryanc-me/odoo-api-rs/ci.yaml?style=flat-square" height="20">](https://github.com/ryanc-me/odoo-api-rs/actions?query=branch%3Amaster)

The `odoo-api` odoo-api is a Rust library crate that provides a user-friendly interface
to interact with the Odoo JSONRPC and ORM APIs, while preserving strong typing. It
includes both async and blocking support out of the box, and allows users to provide
their own implementations if needed.

See the [Example](#example) section below for a brief example, or the [`client`](https://docs.rs/odoo-api/latest/odoo_api/client/index.html) module for more in-depth examples.

### Features

 - **Strong typing**: `odoo-api` prioritizes the use of concrete types wherever
    possible, rather than relying on generic `json!{}` calls.
 - **Async and blocking support**: the library provides both async and blocking
    HTTP impls via [`reqwest`](https://docs.rs/reqwest/latest/reqwest/), and allows users to easily provide their own HTTP
    impl via a shim closure.
 - **JSONRPC API support**: including database management (create, duplicate, etc),
    translations, and generic `execute` and `execute_kw`
 - **ORM API support**: including user-friendly APIs for the CRUD, `search_read`,
    security rule checking, and more
 - **Types-only**: allowing you to include this library for its types only. See
    [Types Only](#types-only) below for more info

#### Supported API Methods

See the [`service`](https://docs.rs/odoo-api/latest/odoo_api/service/index.html) module for a full list of supported API methods.

#### Bring Your Own Requests

Do you already have an HTTP library in your dependencies (e.g., `reqwest`)?

The `odoo-api` crate allows you to use your existing HTTP library by writing a
simple shim closure. See [`client::ClosureAsync`](https://docs.rs/odoo-api/latest/odoo_api/client/struct.ClosureAsync.html) or [`client::ClosureBlocking`](https://docs.rs/odoo-api/latest/odoo_api/client/struct.ClosureBlocking.html)
for more info.

#### Types Only

The crate offers a `types-only` feature. When enabled, the library only exposes
the API request & response types, along with `Serialize` and `Deserialize` impls.
The async/blocking impls (and the `reqwest` dependency) are dropped when this
feature is active.

See the [`jsonrpc`](https://docs.rs/odoo-api/latest/odoo_api/jsonrpc/index.html) module for information on `types-only`.

### Example

<br />
Add the following to your `Cargo.toml`:

```toml
[dependencies]
odoo_api = "0.2"
```

Then make your requests:
```rust
use odoo_api::{OdooClient, jvec, jmap};

// build the client
let url = "https://odoo.example.com";
let mut client = OdooClient::new_reqwest_async(url)?;

// authenticate with `some-database`
let mut client = client.authenticate(
    "some-database",
    "admin",
    "password",
).await?;

// fetch a list of users with the `execute` method
let users = client.execute(
    "res.users",
    "search",
    jvec![
        [["active", "=", true], ["login", "!=", "__system__"]]
    ]
).send().await?;

// fetch the login and partner_id fields from user id=1
let info = client.execute_kw(
    "res.users",
    "read",
    jvec![[1]],
    jmap!{
        "fields": ["login", "partner_id"]
    }
).send().await?;

// create 2 new partners with the `create` ORM method
let partners = client.create(
    "res.partner",
    jvec![{
        "name": "Alice",
        "email": "alice@example.com",
        "phone": "555-555-5555",
    }, {
        "name": "Bob",
        "email": "bob@example.com",
        "phone": "555-555-5555",
    }]
).send().await?;

// fetch a list of databases
let databases = client.db_list(false).send().await?;

// fetch server version info
let version_info = client.common_version().send().await?;
```
