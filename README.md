# odoo-api

[<img alt="github" src="https://img.shields.io/badge/github-ryanc--me/odoo--api--rs-master?style=flat-square&logo=github&color=4078c0" height="20">](https://github.com/ryanc-me/odoo-api-rs)
[<img alt="crates.io" src="https://img.shields.io/crates/v/odoo-api?style=flat-square&logo=rust&color=f9f7ec" height="20">](https://crates.io/crates/odoo-api)
[<img alt="docs.rs" src="https://img.shields.io/docsrs/odoo-api?style=flat-square&logo=docs.rs" height="20">](https://docs.rs/odoo-api/)
[<img alt="docs.rs" src="https://img.shields.io/github/actions/workflow/status/ryanc-me/odoo-api-rs/ci.yaml?style=flat-square" height="20">](https://github.com/ryanc-me/odoo-api-rs/actions?query=branch%3Amaster)

## odoo_api
The `odoo_api` crate provides a type-safe and full-coverage implementation
of the Odoo JSON-RPC API, including ORM and Web methods. It supports sessioning,
multi-database, async and blocking via [`reqwest`](https://docs.rs/reqwest/latest/reqwest/), and bring-your-own requests.

### API Methods

For a full list of supported API methods, see [`odoo_api::service`](https://docs.rs/odoo-api/latest/odoo_api/service/index.html).

### Bring your own requests

By default, `odoo_api` uses [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) as its HTTP implementation. It is also
possible to provide your own HTTP implementation (see [`OdooClient`](https://docs.rs/odoo-api/latest/odoo_api/struct.OdooClient.html) for more info).

### Example

To use the default [`reqwest`](https://docs.rs/reqwest/latest/reqwest/) implementation, add this to your `Cargo.toml`:

```toml
[dependencies]
odoo_api = "0.2"
```

Then make your requests:
```rust
use odoo_api::{OdooClient, jvec, jmap};

// build the client and authenticate
let url = "https://demo.odoo.com";
let client = OdooClient::new_reqwest_async(url)?
    .authenticate(
        "some-database",
        "admin",
        "password",
    ).await?;

// fetch a list of users
let users = client.execute(
    "res.users",
    "search",
    jvec![]
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

// fetch a list of databases
let databases = client.db_list(false).send().await?;

// fetch server version info
let version_info = client.common_version().send().await?;
```
