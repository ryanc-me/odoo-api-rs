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
/// // Manually
/// let mut args = Vec::<Value>::new();
/// args.push(json!([1, 2, 3]));
/// args.push(json!(["id", "login"]))
///
/// let request = client.execute(
///     "res.users",
///     "read",
///     args,
/// );
///
/// // With jvec![]:
/// let request = client.execute(
///     "res.users",
///     "read",
///     jvec![
///         [1, 2, 3],
///         ["id", "login"]
///     ]
/// );
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
/// // Manually
/// let mut kwargs = Map::<String, Value>::new();
/// kwargs.insert("domain", json!([["name", "ilike", "admin"]]));
/// kwargs.insert("fields": json!(["id", "login"]));
///
/// let request = client.execute(
///     "res.users",
///     "search_read",
///     jvec![],
///     kwargs,
/// );
///
/// // With jmap!{}:
/// let request = client.execute(
///     "res.users",
///     "search_read",
///     jvec![],
///     jmap!{
///         "domain": [["name", "ilike", "admin"]],
///         "fields": ["id", "login"]
///     }
/// );
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
