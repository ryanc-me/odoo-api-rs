use std::fmt::Debug;
use serde::{Serialize, Serializer, ser::SerializeStruct, Deserialize, de::DeserializeOwned};
use serde_json::{Map, Value, to_string, to_string_pretty, to_value};


mod error;
pub use error::*;

pub mod types;

#[cfg(all(feature = "async"))]
pub mod asynch;

#[cfg(feature = "blocking")]
pub mod blocking;


/// A JSON-RPC call id
pub type JsonRpcId = u32;

/// An Odoo id
pub type OdooID = u32;

/// A string representing the JSON-RPC version
/// 
/// At the time of writing, this is always set to "2.0"
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
    /// The JSON-RPC call version (this is always "2.0")

    /// Odoo JSON-RCP API version 2.0
    #[serde(rename = "2.0")]
    V2,
}

/// A string representing the JSON-RPC "method"
/// 
/// At the time of writing, this is always set to "call"
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcMethod {
    #[serde(rename = "call")]
    Call,
}

/// An Odoo JSON-RPC API request
///
/// This struct represents the base JSON data, and is paramterized over the
/// [`OdooApiMethod`] (e.g., the `param` field will be an `OdooApiMethod`)
///
/// See: [base/controllers/rpc.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/addons/base/controllers/rpc.py#L154-L157)
/// See also: [odoo/http.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/http.py#L347-L368)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OdooApiRequest<T> where T: OdooApiMethod + Serialize + Debug + PartialEq {
    /// The JSON-RPC version (`2.0`)
    pub version: JsonRpcVersion,

    /// The JSON-RPC method (`call`)
    pub method: JsonRpcMethod,

    /// The request id
    /// 
    /// This is not used for any stateful behaviour on the Odoo/Python side
    pub id: JsonRpcId,

    /// The request params (service, method, and arguments)
    pub params: JsonRpcRequestParams<T>,
}

/// A container struct for the API request data
/// 
/// This struct is used to implement a custom [`Serialize`](serde::Serialize).
/// The struct is actually serialized into JSON as:
/// ```jsonc
/// {
///     "service": "xxx"
///     "method": "xxx",
///     "args": args
/// }
/// ```
#[derive(Debug, Deserialize, PartialEq)]
pub struct JsonRpcRequestParams<T> where T: OdooApiMethod + Serialize + Debug + PartialEq {
    pub args: T
}

impl<T> Serialize for JsonRpcRequestParams<T> where T: OdooApiMethod + Serialize {
    fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("args", 3)?;
        let (service, method) = self.args.describe_odoo_api_method();
        state.serialize_field("service", service)?;
        state.serialize_field("method", method)?;
        state.serialize_field("args", &self.args)?;
        state.end()
    }
}


/// An Odoo JSON-RPC API response
/// 
/// This struct represents the base JSON data, and is paramterized over the
/// *request* [`OdooApiMethod`]. The deserialization struct is chosen by
/// looking at the associated type [`OdooApiMethod::Response`].
/// 
/// See: [odoo/http.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/http.py#L1805-L1841)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum OdooApiResponse<T>
where
    T: OdooApiMethod + Serialize + Debug + PartialEq
{
    /// A successful Odoo API response
    Success(JsonRpcResponseSuccess<T>),

    /// A failed Odoo API response
    Error(JsonRpcResponseError),
}

impl<T: OdooApiMethod + Serialize + Debug + PartialEq> OdooApiResponse<T> {
    /// Convert the response struct into a [`serde_json::Value`]
    pub fn to_json_value(&self) -> serde_json::Result<Value> {
        to_value(self)
    }

    /// Convert the response struct into a "minified" string
    pub fn to_json_string(&self) -> serde_json::Result<String> {
        to_string(self)
    }

    /// Convert the response struct into a "prettified" string
    pub fn to_json_string_pretty(&self) -> serde_json::Result<String> {
        to_string_pretty(self)
    }
}

/// A successful Odoo API response
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcResponseSuccess<T>
where
    T: OdooApiMethod + Serialize + Debug + PartialEq
{
    /// The JSON-RPC version (`2.0`)
    pub jsonrpc: JsonRpcVersion,

    /// The request id
    /// 
    /// This is not used for any stateful behaviour on the Odoo/Python side
    pub id: JsonRpcId,

    /// The response data, parameterized on the *request* [`OdooApiMethod::Response`]
    /// associated type.
    pub result: T::Response,
}

/// A failed Odoo API response
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcResponseError {
    /// The JSON-RPC version (`2.0`)
    pub jsonrpc: JsonRpcVersion,

    /// The request id
    /// 
    /// This is not used for any stateful behaviour on the Odoo/Python side
    pub id: JsonRpcId,

    /// A struct containing the error information
    pub error: JsonRpcError,
}

/// A struct representing the high-level error information
/// 
/// See: [odoo/http.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/http.py#L1805-L1841)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    /// The error code. Currently hardcoded to `200`
    pub code: u32,

    /// The error "message". This is a short string indicating the type of
    /// error. Some examples are:
    ///  * `Odoo Server Error`
    ///  * `404: Not Found`
    ///  * `Odoo Session Expired`
    pub message: String,

    /// The actual error data
    pub data: JsonRpcErrorData,
}

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{:?}", self)
    }
}

/// A struct representing the low-level error information
/// 
/// See: [odoo/http.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/http.py#L375-L385)
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcErrorData {
    /// The module? and type of the object where the exception was raised
    /// 
    /// For example:
    ///  * `builtins.TypeError`
    ///  * `odoo.addons.account.models.account_move.AccountMove`
    pub name: String,

    /// The Python exception stack trace
    pub debug: String,

    /// The Python exception message (e.g. `str(exception)`)
    pub message: String,

    /// The Python exception arguments (e.g. `excetion.args`)
    pub arguments: Vec<Value>,

    /// The Python exception context (e.g. `excetion.context`)
    pub context: Map<String, Value>,
}


impl<T: OdooApiMethod + Serialize + Debug + PartialEq> OdooApiRequest<T> {
    /// Convert the request struct into a [`serde_json::Value`]
    pub fn to_json_value(&self) -> serde_json::Result<Value> {
        to_value(self)
    }

    /// Convert the request struct into a "minified" string
    pub fn to_json_string(&self) -> serde_json::Result<String> {
        to_string(self)
    }

    /// Convert the request struct into a "prettified" string
    pub fn to_json_string_pretty(&self) -> serde_json::Result<String> {
        to_string_pretty(self)
    }

    /// Parse a JSON string into the [`OdooApiMethod::Response`] associated type
    pub fn parse_json_response(&self, json_data: &str) -> serde_json::Result<OdooApiResponse<T>> {
        self.params.args.parse_json_response(json_data)
    }
}




/// A trait implemented by the "request" structs
///
/// This trait serves a few purposes:
///  1. Create a link between the request and response structs (e.g., [`Execute`](crate::types::object::Execute) and [`ExecuteResponse`](crate::types::object::ExecuteResponse))
///  2. Describe the request (e.g. service: `object`, method: `execute`)
///  3. Provide a response-parsing function
pub trait OdooApiMethod where Self: Sized + Serialize + Debug + PartialEq {
    /// The response type (e.g., the [`ExecuteResponse`](crate::types::object::ExecuteResponse) for [`Execute`](crate::types::object::Execute))
    type Response: Sized + Serialize + DeserializeOwned + Debug + PartialEq + TryFrom<String> + TryFrom<Value>;

    /// Describes the Odoo API method (including the service)
    /// 
    /// The Odoo API is split into "services" and "methods".
    /// 
    /// For example, his function is responsible for returning the `"common"`
    /// and `"version"` below:
    /// ```jsonc
    /// {
    ///     "jsonrpc": "2.0",
    ///     "method": "call",
    ///     "params": {
    ///         // the "service"
    ///         "service": "common",
    /// 
    ///         // the "method"
    ///         "method": "version",
    /// 
    ///         "args": []
    ///     }
    /// }
    /// ```
    ///
    fn describe_odoo_api_method(&self) -> (&'static str, &'static str);

    /// Parse some JSON string data into an [`OdooApiResponse`](crate::jsonrpc::OdooApiRequest) object
    ///
    /// Internally, `OdooApiResponse` uses the [`Response`](crate::jsonrpc::OdooApiMethod::Response) associated type to
    /// decide how to deserialize the JSON data.
    fn parse_json_response(&self, json_data: &str) -> serde_json::Result<OdooApiResponse<Self>>;
}

// use std::convert::{TryFrom, TryInto};

// impl<T> TryFrom<&str> for T
// where
//     T: OdooApiMethod
// {
//     type Error = Error;

//     fn try_from(value: &str) -> ::std::result::Result<T::Response, Self::Error> {

//         Ok(())
//     }
// }
// impl<T: OdooApiMethod> TryInto<T> for &str
// {
//     type Error = Error;

//     fn try_into(value: &str) -> ::std::result::Result<T, Self::Error> {

//         Ok(())
//     }
// }
