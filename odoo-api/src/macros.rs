//! Internal module to store macro_rules!() macros

// Import Value and Map so the doc comments render properly
#[allow(unused_imports)]
use serde_json::{Map, Value};

/// Helper macro to build a [`Vec<Value>`]
///
/// This is useful when using any of the API methods that require a `Vec<Value>`,
/// as [`serde_json`] doesn't have a way to build these.
///
/// ## Example:
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> odoo_api::client::error::Result<()> {
/// # use serde_json::{json, Value};
/// # use odoo_api::{jvec, jmap};
/// # use odoo_api::{OdooClient};
/// # let client = OdooClient::new_reqwest_blocking("https://demo.odoo.com")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// // Manually
/// let mut args = Vec::<Value>::new();
/// args.push(json!([1, 2, 3]));
/// args.push(json!(["id", "login"]));
///
/// let request = client.execute(
///     "res.users",
///     "read",
///     args,
/// ).send()?;
///
/// // With jvec![]:
/// let request = client.execute(
///     "res.users",
///     "read",
///     jvec![
///         [1, 2, 3],
///         ["id", "login"]
///     ]
/// ).send()?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! jvec {
    [$($v:tt),*] => {
        {
            let mut vec = ::std::vec::Vec::<serde_json::Value>::new();
            $(
                vec.push(::serde_json::json!($v));
            )*
            vec
        }
    };
    () => { compiler_error!("")};
}

/// Helper macro to build a [`Map<String, Value>`]
///
/// This is useful when using any of the API methods that require a `Map<String, Value>`,
/// as [`serde_json`] doesn't have a way to build these.
///
/// ## Example:
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> odoo_api::client::error::Result<()> {
/// # use serde_json::{json, Value, Map};
/// # use odoo_api::{jvec, jmap};
/// # use odoo_api::{OdooClient};
/// # let client = OdooClient::new_reqwest_blocking("https://demo.odoo.com")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// // Manually
/// let mut kwargs = Map::<String, Value>::new();
/// kwargs.insert("domain".into(), json!([["name", "ilike", "admin"]]));
/// kwargs.insert("fields".into(), json!(["id", "login"]));
///
/// let request = client.execute_kw(
///     "res.users",
///     "search_read",
///     jvec![],
///     kwargs,
/// ).send()?;
///
/// // With jmap!{}:
/// let request = client.execute_kw(
///     "res.users",
///     "search_read",
///     jvec![],
///     jmap!{
///         "domain": [["name", "ilike", "admin"]],
///         "fields": ["id", "login"]
///     }
/// ).send()?;
/// # Ok(())
/// # }
/// ```
#[macro_export]
macro_rules! jmap {
    {$($k:tt: $v:tt),*} => {
        {
            let mut map = ::serde_json::Map::<String, ::serde_json::Value>::new();
            $(
                map.insert($k.into(), ::serde_json::json!($v));
            )*
            map
        }
    };
    () => { compiler_error!("")};
}

/// Helper macro to build a [`Vec<String>`]
///
/// Quite a few ORM methods take [`Vec<String>`] as an argument. Using the built-in
/// `vec![]` macro requires that each element is converted into a `String`, which
/// is very cumbersome.
///
/// Using this macro, we can write:
/// ```
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() {
/// # use odoo_api::svec;
/// let fields = svec!["string", "literals", "without", "to_string()"];
/// # }
/// ```
#[macro_export]
macro_rules! svec {
    [$($v:tt),*] => {
        {
            let mut vec = ::std::vec::Vec::<String>::new();
            $(
                vec.push($v.to_string());
            )*
            vec
        }
    };
    () => { compiler_error!("")};
}
