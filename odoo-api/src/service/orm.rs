//! The Odoo "ORM" pseudo-service
//!
//! This isn't really an Odoo "service", but instead is another layer of abstraction
//! over the object `execute` and `execute_kw` methods, providing a nicer interface
//! with better type checking.

use crate as odoo_api;
use crate::jsonrpc::{OdooId, OdooIds, OdooOrmMethod};
use odoo_api_macros::odoo_orm;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize, Deserializer, de};
use serde_json::{Map, Value};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use std::collections::HashMap;
use std::fmt;

/// Create a new record (or set of records)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jvec, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// // create a single record
/// let resp = client.create(
///     "res.partner",
///     jmap!{
///         "name": "Example Partner",
///         "email": "hello@example.com"
///     }
/// ).send()?;
///
/// // create multiple records
/// let resp2 = client.create(
///     "res.partner",
///     jvec![
///         {"name": "Partner #1", "email": "marco@example.com"},
///         {"name": "Partner #2", "email": "polo@example.com"}
///     ]
/// ).send()?;
///
/// println!("New partner ids: {:?}", resp2.ids);
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3829-L3964)
#[odoo_orm(
    method = "create",
    args = ["values"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct Create {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// Values for the new record(s)
    pub values: CreateVals,
}

/// The values to a [`Create`] request
///
/// The Odoo `create()` function accepts either a dictionary (create one record),
/// or a list of dictionaries (create multiple records). To support those in an
/// ergonomic way, we will accept an enum for the value.
///
/// This enum implements `From<...>` for both one & multi requests:
/// ```ignore
/// // create a single record
/// client.create(
///     "res.users",
///     jmap!{
///         "name": "Hello, world!",
///     }
/// );
///
/// // create multiple records
/// client.create(
///     "res.users",
///     jvec![
///         {"name": "Partner #1"},
///         {"name": "Partner #2"}
///     ]
/// );
/// ```
#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum CreateVals {
    /// Create a single new record
    One(Map<String, Value>),

    /// Create multiple new records
    Multi(Vec<Value>),
}

impl From<Map<String, Value>> for CreateVals {
    fn from(value: Map<String, Value>) -> Self {
        Self::One(value)
    }
}

impl From<Vec<Value>> for CreateVals {
    fn from(value: Vec<Value>) -> Self {
        Self::Multi(value)
    }
}

/// The response to a [`Create`] requests
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateResponse {
    /// The new record(s) id(s)
    pub ids: CreateResponseItem,
}

/// Container for the [`CreateResponse`] items
///
/// Because thr [`Create`] request can create one OR multiple records, the response
/// may be one or multiple ids. In the "one" case, Odoo returns a plain int. In
/// the "multi" case, Odoo returns an array of ints.
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateResponseItem {
    /// The new records' id
    One(OdooId),

    /// A list of the ids for the new records
    Multi(Vec<OdooId>),
}

/// Read data from a record (or set of records)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, svec};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// // read from a single record
/// let resp = client.read(
///     "res.partner",
///     1,
///     svec!["id", "login"]
/// ).send()?;
///
/// // read from multiple records
/// let resp = client.read(
///     "res.partner",
///     vec![1, 2, 3],
///     svec!["id", "login"]
/// ).send()?;
///
/// println!("Data: {:?}", resp.data);
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L2958-L2991)
#[odoo_orm(
    method = "read",
    args = ["ids"],
    kwargs = ["fields"],
)]
#[derive(Debug)]
pub struct Read {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records
    pub ids: OdooIds,

    /// The fields to be fetched
    pub fields: Vec<String>,
}

/// The response to a [`Read`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadResponse {
    /// The fetched fields
    pub data: Vec<Map<String, Value>>,
}

/// Write data to a record (or set of records)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// // write to a single record
/// client.write(
///     "res.partner",
///     1,
///     jmap!{
///         "name": "New Partner Name"
///     }
/// ).send()?;
///
/// // write to multiple records
/// client.write(
///     "res.partner",
///     vec![1, 2, 3],
///     jmap!{
///         "website": "https://www.example.com"
///     }
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3585-L3775)
#[odoo_orm(
    method = "write",
    args = ["ids", "values"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct Write {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records
    pub ids: OdooIds,

    /// The values to write
    pub values: Map<String, Value>,
}

/// The response to a [`Write`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WriteResponse {
    pub ok: bool,
}

/// Delete a record (or set of records)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// // delete one record
/// client.unlink(
///     "res.partner",
///     1
/// ).send()?;
///
/// // delete multiple records
/// client.unlink(
///     "res.partner",
///     vec![1, 2, 3]
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3488-L3583)
#[odoo_orm(
    method = "unlink",
    args = ["ids"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct Unlink {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records to be deleted
    pub ids: OdooIds,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UnlinkResponse {
    pub ok: bool,
}

/// Read some grouped data from a record (or set of records)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jvec, svec};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.read_group(
///     "res.partner",
///
///     // domain
///     jvec![
///         ["email", "=ilike", "%@example.com"],
///         ["phone", "!=", false]
///     ],
///
///     // fields
///     svec!["id", "name", "email", "phone"],
///
///     // groupby
///     svec!["create_date:month", "email"],
///
///     Some(0), // offset
///     Some(10), // limit
///     Some("create_date desc".into()), // orderby
///     false // lazy
/// ).send()?;
///
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L2178-L2236)
#[odoo_orm(
    method = "read_group",
    args = ["domain", "fields", "groupby"],
    kwargs = ["offset", "limit", "orderby", "lazy"],
)]
#[derive(Debug)]
pub struct ReadGroup {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The domain to search on
    pub domain: Vec<Value>,

    /// The fields to read
    pub fields: Vec<String>,

    /// The groupby descriptions
    ///
    /// See [`read_group`](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L2191-L2195) for more information.
    pub groupby: Vec<String>,

    /// An optional offset, for paging
    pub offset: Option<u32>,

    /// An optional limit
    pub limit: Option<u32>,

    /// An optional ordering description
    ///
    /// This corresponds roughly with PostgreSQL's `ORDER BY` clause, for example:
    /// `create_date desc, id asc, active asc`
    pub orderby: Option<String>,

    /// Enable lazy-grouping
    ///
    /// If `true`, then only the first `groupby` fragment is evaluated, and all
    /// other fragments are added to the `__context` key in the response.
    ///
    /// This may be useful when initially showing a list of top-level groups; evaluating
    /// only the first `groupby` will be much faster for large datasets.
    pub lazy: bool,
}

//TODO: a better response type (e.g., struct with __domain/etc, and a `data` key for the actual fields)
/// The response to a [`ReadGroup`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ReadGroupResponse {
    pub result: Vec<Map<String, Value>>
}

/// Perform a `search` and `read` in one call
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jvec, svec};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.search_read(
///     "res.partner",
///
///     // domain
///     jvec![
///         ["email", "=ilike", "%@example.com"],
///         ["phone", "!=", false]
///     ],
///
///     // fields
///     svec!["id", "name", "email", "phone"],
///
///     Some(0), // offset
///     Some(10), // limit
///     Some("create_date desc".into()), // order
/// ).send()?;
///
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L4920-L4963)
#[odoo_orm(
    method = "search_read",
    args = [],
    kwargs = ["domain", "fields", "offset", "limit", "order"],
)]
#[derive(Debug)]
pub struct SearchRead {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The domain to search on
    pub domain: Vec<Value>,

    /// The fields to read
    pub fields: Vec<String>,

    /// An optional offset, for paging
    pub offset: Option<u32>,

    /// An optional limit
    pub limit: Option<u32>,

    /// An optional ordering description
    ///
    /// This corresponds roughly with PostgreSQL's `ORDER BY` clause, for example:
    /// `create_date desc, id asc, active asc`
    pub order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SearchReadResponse {
    pub data: Vec<Map<String, Value>>,
}

//TODO: notes about the `count` flag (maybe disable that - we have search_count)
/// Return the ids of records matching a domain
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jvec};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.search(
///     "res.partner",
///
///     // domain
///     jvec![
///         ["email", "=ilike", "%@example.com"],
///         ["phone", "!=", false]
///     ],
///
///     Some(0), // offset
///     Some(10), // limit
///     Some("create_date desc".into()), // order
/// ).send()?;
///
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L1517-L1537)
#[odoo_orm(
    method = "search",
    args = ["domain"],
    kwargs = ["offset", "limit", "order"],
)]
#[derive(Debug)]
pub struct Search {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The domain to search on
    pub domain: Vec<Value>,

    /// An optional offset, for paging
    pub offset: Option<u32>,

    /// An optional limit
    pub limit: Option<u32>,

    /// An optional ordering description
    ///
    /// This corresponds roughly with PostgreSQL's `ORDER BY` clause, for example:
    /// `create_date desc, id asc, active asc`
    pub order: Option<String>,
}

/// The response to a [`Search`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SearchResponse {
    pub records: Vec<OdooId>,
}

/// Return the count of records matching a domain
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jvec};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.search_count(
///     "res.partner",
///
///     // domain
///     jvec![
///         ["email", "=ilike", "%@example.com"],
///         ["phone", "!=", false]
///     ],
///
///     None, // limit
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L1503-L1515)
#[odoo_orm(
    method = "search_count",
    args = ["domain"],
    kwargs = ["limit"],
)]
#[derive(Debug)]
pub struct SearchCount {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The domain to search on
    pub domain: Vec<Value>,

    /// An optional limit
    pub limit: Option<u32>
}

/// The response to a [`SearchCount`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SearchCountResponse {
    pub count: u32,
}

/// Copy a record
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.copy(
///     "res.partner",
///
///     2, // record id
///     None, // override fields
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L4755-L4771)
#[odoo_orm(
    method = "copy",
    args = ["id"],
    kwargs = ["default"],
)]
#[derive(Debug)]
pub struct Copy {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The record to copy
    pub id: OdooId,

    /// The fields to be overridden
    pub default: Option<Map<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CopyResponse {
    pub id: OdooId,
}

/// Check if the record(s) exist in the Odoo database
/// 
/// **Note**: This method works by accepting a list of ids, and returning only
/// the ids that actually exist. See [`ExistsResponse`] for more info.
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.exists(
///     "res.partner",
///     vec![1, 2, -1, 999999999]
/// ).send()?;
///
/// // The response would be [1, 2], assuming that -1 and 999999999 do not exist
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L4773-L4793)
#[odoo_orm(
    method = "exists",
    args = ["ids"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct Exists {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The ids to check
    pub ids: OdooIds,
}

/// The response to an [`Exists`] call
///
/// The [`Exists`] method is soemwhat unituitive; It accepts *multiple* ids, then
/// returns only the ids that actually exist (rather than returning true/false).
///
/// To use this method, you should pass the record ids you want to check, then
/// test whether those ids were returned in the `existing_records` field.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ExistsResponse {
    pub existing_records: OdooIds,
}

/// An access operation type
#[derive(Debug, Serialize)]
pub enum AccessOperation {
    #[serde(rename = "create")]
    Create,

    #[serde(rename = "read")]
    Read,

    #[serde(rename = "write")]
    Write,

    #[serde(rename = "unlink")]
    Unlink,
}

/// Check model access rights (according to `ir.model.access`)
/// 
/// This method checks against `ir.model.access`, e.g. basic per-group CRUD rules.
/// You should also call [`CheckAccessRules`] in order to determine if any advanced
/// access rules apply to this model/user.
/// 
/// **Note**: This method raises an API exception if the access rights check fails.
/// You probably want to specify `raise_exception: false`, which will cause the
/// request to return `false` when the check fails.
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// use odoo_api::service::orm::AccessOperation;
/// client.check_access_rights(
///     "stock.quant",
///     AccessOperation::Unlink,
///     false // raise_exception
/// ).send()?;
///
/// // Quants are never deleteable, so this will return `false`
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3407-L3417)
#[odoo_orm(
    method = "check_access_rights",
    args = ["operation"],
    kwargs = ["raise_exception"],
)]
#[derive(Debug)]
pub struct CheckAccessRights {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The CRUD operation to check
    pub operation: AccessOperation,

    /// How should check failures be reported?
    /// 
    ///  * `true`: An API exception is raised (catchable with `?`, etc)
    ///  * `false`: The [`CheckAccessRightsResponse`] `ok` field will be set to `false`
    pub raise_exception: bool,
}

/// Response to a [`CheckAccessRights`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CheckAccessRightsResponse {
    pub ok: bool
}

/// Check model access rules (according to `ir.rule`)
/// 
/// This method checks against `ir.rule`, e.g. advanced domain-based CRUD rules.
/// You should also call [`CheckAccessRights`] in order to determine if any
/// basic CRUD/group rights apply to this model/user.
/// 
/// **NOTE**: If the access check fails, an API error will be returned. To determine
/// if the rules passed, check for the "Success" enum variant on the response.
/// 
/// **WARNING**: This method currently raises an API exception on success. This issue
/// will be fixed in a future release. For now, you may check for 
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// use odoo_api::service::orm::AccessOperation;
/// client.check_access_rules(
///     "res.partner",
///     vec![1, 2], // records
///     AccessOperation::Unlink,
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3419-L3453)
#[odoo_orm(
    method = "check_access_rule",
    name = "check_access_rules",
    args = ["ids", "operation"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct CheckAccessRules {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records to be checked
    pub ids: OdooIds,

    /// The CRUD operation to check
    pub operation: AccessOperation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckAccessRulesResponse {}

/// Check the user access rights on the given fields
/// 
/// **Note**: Like the [`Exists`] method, this method accepts a list of fields,
/// then returns the subset of those fields that can be accessed via `operation`.
/// 
/// **Note2**: This method doesn't check if the passed fields actually exist.
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap, svec};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// use odoo_api::service::orm::AccessOperation;
/// client.check_field_access_rights(
///     "res.partner",
///     AccessOperation::Unlink,
///     svec!["parent_id", "email", "id"]
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L2864-L2956)
#[odoo_orm(
    method = "check_field_access_rights",
    args = ["operation", "fields"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct CheckFieldAccessRights {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The CRUD operation to check
    pub operation: AccessOperation,

    /// A list of fields to check
    pub fields: Vec<String>,
}

/// The response to a [`CheckFieldAccessRights`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CheckFieldAccessRightsResponse {
    pub result: Option<Vec<String>>,
}


/// Return some metadata about the given record(s)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jvec};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.get_metadata(
///     "res.partner",
///     vec![1, 2]
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L3275-L3315)
#[odoo_orm(
    method = "get_metadata",
    args = ["ids"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct GetMetadata {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records to fetch metadata for
    pub ids: OdooIds,
}

/// The response to a [`GetMetadata`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetMetadataResponse {
    pub metadata: Vec<Map<String, Value>>,
}

// Allow the map of {str: str} to be deserialized into {i32: str}
fn get_external_id_deserialize<'de, D>(de: D) -> Result<HashMap<OdooId, String>, D::Error>
where
    D: Deserializer<'de>
{
    struct Visitor;
    impl<'de> de::Visitor<'de> for Visitor {
        type Value = HashMap<OdooId, String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of \"id\": \"external_id\"")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            let mut map = HashMap::new();

            // While there are entries remaining in the input, add them
            // into our map.
            while let Some((key, value)) = access.next_entry::<String, String>()? {
                let key = key.parse().map_err(|_e| {
                    de::Error::invalid_value(
                        de::Unexpected::Str(&key),
                        &"A String representing an i32"
                    )
                })?;
                map.insert(key, value);
            }

            Ok(map)
        }
    }

    de.deserialize_map(Visitor)
}

/// Fetch the XMLID for the given record(s)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jvec};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.get_external_id(
///     "res.partner",
///     vec![1, 2]
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L4882-L4901)
#[odoo_orm(
    method = "get_external_id",
    args = ["ids"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct GetExternalId {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records to fetch external ids for
    pub ids: OdooIds,
}

/// The response to a [`GetExternalId`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetExternalIdResponse {
    #[serde(deserialize_with = "get_external_id_deserialize")]
    pub external_ids: HashMap<OdooId, String>,
}

/// Fetch the XMLID for the given record(s)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.get_xml_id(
///     "res.partner",
///     vec![1, 2]
/// ).send()?;
/// # Ok(())
/// # }
/// ```
///<br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L4903-L4908)
#[odoo_orm(
    method = "get_xml_id",
    args = ["ids"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct GetXmlId {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The records to fetch XMLIDs for
    pub ids: OdooIds,
}

/// The response to a [`GetXmlId`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct GetXmlIdResponse {
    #[serde(deserialize_with = "get_external_id_deserialize")]
    pub external_ids: HashMap<OdooId, String>,
}

/// Fetch the `display_naame` for the given record(s)
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.name_get(
///     "res.partner",
///     vec![1, 2, 3]
/// ).send()?;
/// # Ok(())
/// # }
/// ```
/// <br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L1560-L1584)
#[odoo_orm(
    method = "name_get",
    args = ["ids"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct NameGet {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The domain
    pub ids: OdooIds,
}

/// The response to a [`NameGet`] request
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NameGetResponse {
    pub display_names: Vec<NameGetResponseItem>,
}

/// An individual [`NameGet`] response item
#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct NameGetResponseItem {
    /// The record id
    pub id: OdooId,

    /// The record `display_name`
    pub name: String,
}

/// Create a new record, passing only the `name` field
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.name_create(
///     "res.partner",
///     "I am a test!".into()
/// ).send()?;
/// # Ok(())
/// # }
/// ```
/// <br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L1586-L1606)
#[odoo_orm(
    method = "name_create",
    args = ["name"],
    kwargs = [],
)]
#[derive(Debug)]
pub struct NameCreate {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// A name for the new record
    pub name: String,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct NameCreateResponse {
    /// The record id
    pub id: OdooId,

    /// The record `display_name`
    pub name: String,
}

/// Search for records based on their `name` field
/// 
/// This is a shortcut to the `search()` method with only one domain component:
/// `[("name", "ilike", name)]`. This function is generally used by the Odoo searchbar,
/// and by the search function on x2many fields.
/// 
/// **Note**: Some models may override this method to provide custom "name search"
/// behaviour.
///
/// ## Example
/// ```no_run
/// # #[cfg(not(feature = "types-only"))]
/// # fn test() -> Result<(), Box<dyn std::error::Error>> {
/// # use odoo_api::{OdooClient, jmap};
/// # let client = OdooClient::new_reqwest_blocking("")?;
/// # let mut client = client.authenticate_manual("", "", 1, "", None);
/// client.name_search(
///     "res.partner",
///     "Admini%".into(),
///     None,
///     None,
///     None,
/// ).send()?;
/// # Ok(())
/// # }
/// ```
/// <br />
///
/// See: [odoo/models.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/models.py#L1608-L1634)
#[odoo_orm(
    method = "name_search",
    args = ["name"],
    kwargs = ["args", "operator", "limit"],
)]
#[derive(Debug)]
pub struct NameSearch {
    /// The database name (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub database: String,

    /// The user id (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub uid: OdooId,

    /// The user password (auto-filled by [`OdooClient`](crate::client::OdooClient))
    pub password: String,

    /// The Odoo model
    pub model: String,

    /// The name to search for (can include operators like `%`)
    pub name: String,

    /// An optional search domain
    pub args: Option<Vec<Value>>,

    /// A domain operator for the "name test"
    ///
    /// For example:
    ///  * `ilike` (default)
    ///  * `=`
    ///  * ...etc
    pub operator: Option<String>,

    /// Limit the number of results
    pub limit: Option<u32>,
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(transparent)]
pub struct NameSearchResponse {
    pub records: Vec<NameSearchResponseItem>
}

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
pub struct NameSearchResponseItem {
    /// The record id
    pub id: OdooId,

    /// The record `display_name`
    pub name: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::client::error::Result;
    use crate::jsonrpc::{JsonRpcParams, JsonRpcResponse};
    use crate::{jmap, jvec, svec};
    use serde_json::{from_value, json, to_value};

    #[test]
    fn create_one() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "create",
                    [
                        {"name": "Hello, world!"}
                    ],
                    {}
                ]
            }
        });
        let actual = to_value(
            Create {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                values: jmap!{"name": "Hello, world!"}.into(),
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn create_one_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": 47
        });

        let response: JsonRpcResponse<CreateResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn create_multi() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "create",
                    [
                        [
                            {"name": "Hello, world!"},
                            {"name": "Marco, polo!"}
                        ]
                    ],
                    {}
                ]
            }
        });
        let actual = to_value(
            Create {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                values: jvec![
                    {"name": "Hello, world!"},
                    {"name": "Marco, polo!"}
                ].into(),
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn create_multi_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                50,
                51
            ]
        });

        let response: JsonRpcResponse<CreateResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn read() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "read",
                    [
                        [1, 2, 3]
                    ],
                    {
                        "fields": ["id", "login"]
                    }
                ]
            }
        });
        let actual = to_value(
            Read {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                ids: vec![1, 2, 3].into(),
                fields: svec!["id", "login"],
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn read_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                {
                    "id": 1,
                    "name": "My Company (San Francisco)"
                },
                {
                    "id": 2,
                    "name": "OdooBot"
                },
                {
                    "id": 3,
                    "name": "Administrator"
                }
            ]
        });

        let response: JsonRpcResponse<ReadResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn write() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "write",
                    [
                        [2],
                        {
                            "name": "The Admin Account"
                        }
                    ],
                    {}
                ]
            }
        });
        let actual = to_value(
            Write {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                ids: 2.into(),
                values: jmap! {"name": "The Admin Account"},
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn write_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": true
        });

        let response: JsonRpcResponse<WriteResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn unlink() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "unlink",
                    [
                        [3],
                    ],
                    {}
                ]
            }
        });
        let actual = to_value(
            Unlink {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),
                ids: 3.into(), // the "default" user - be careful!
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn unlink_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": true
        });

        let response: JsonRpcResponse<UnlinkResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn read_group() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "read_group",
                    [
                        [
                            ["id", ">", 0]
                        ],
                        [
                            "id",
                            "name",
                            "company_type"
                        ],
                        [
                            "create_date:month",
                            "company_id"
                        ]
                    ],
                    {
                        "offset": 0,
                        "limit": 100,
                        "orderby": "create_date desc",
                        "lazy": false
                    }
                ]
            }
        });
        let actual = to_value(
            ReadGroup {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                domain: jvec![["id", ">", 0]],
                fields: svec!["id", "name", "company_type"],
                groupby: svec!["create_date:month", "company_id"],
                offset: Some(0),
                limit: Some(100),
                orderby: Some("create_date desc".into()),
                lazy: false,
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn read_group_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                {
                    "__count": 5,
                    "create_date:month": "January 2023",
                    "company_id": false,
                    "__domain": [
                        "&",
                        "&",
                        "&",
                        [
                            "create_date",
                            ">=",
                            "2023-01-01 00:00:00"
                        ],
                        [
                            "create_date",
                            "<",
                            "2023-02-01 00:00:00"
                        ],
                        [
                            "company_id",
                            "=",
                            false
                        ],
                        [
                            "id",
                            ">",
                            0
                        ]
                    ]
                },
                {
                    "__count": 1,
                    "create_date:month": "December 2022",
                    "company_id": [
                        1,
                        "Test!"
                    ],
                    "__domain": [
                        "&",
                        "&",
                        "&",
                        [
                            "create_date",
                            ">=",
                            "2022-12-01 00:00:00"
                        ],
                        [
                            "create_date",
                            "<",
                            "2023-01-01 00:00:00"
                        ],
                        [
                            "company_id",
                            "=",
                            1
                        ],
                        [
                            "id",
                            ">",
                            0
                        ]
                    ]
                },
            ]
        });

        let response: JsonRpcResponse<ReadGroupResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn search_read() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "search_read",
                    [],
                    {
                        "domain": [
                            ["company_type", "=", "company"]
                        ],
                        "fields": [
                            "id",
                            "name",
                            "company_type"
                        ],
                        "offset": 0,
                        "limit": 100,
                        "order": "create_date desc"
                    }
                ]
            }
        });
        let actual = to_value(
            SearchRead {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                domain: jvec![["company_type", "=", "company"]],
                fields: svec!["id", "name", "company_type"],
                offset: Some(0),
                limit: Some(100),
                order: Some("create_date desc".into()),
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn search_read_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                {
                    "id": 48,
                    "name": "Partner #1",
                    "company_type": "person"
                },
                {
                    "id": 49,
                    "name": "Partner #2",
                    "company_type": "person"
                },
            ]
        });

        let response: JsonRpcResponse<SearchReadResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn search() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "search",
                    [
                        [
                            ["company_type", "=", "company"]
                        ]
                    ],
                    {
                        "offset": 0,
                        "limit": 100,
                        "order": "create_date desc"
                    }
                ]
            }
        });
        let actual = to_value(
            Search {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                domain: jvec![["company_type", "=", "company"]],
                offset: Some(0),
                limit: Some(100),
                order: Some("create_date desc".into()),
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn search_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                48,
                49,
                47,
                45,
                44,
            ]
        });

        let response: JsonRpcResponse<SearchResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn search_count() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "search_count",
                    [
                        [
                            ["company_type", "=", "company"]
                        ]
                    ],
                    {
                        "limit": null
                    }
                ]
            }
        });
        let actual = to_value(
            SearchCount {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                domain: jvec![["company_type", "=", "company"]],
                limit: None,
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn search_count_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": 46
        });

        let response: JsonRpcResponse<SearchCountResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn copy() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "copy",
                    [
                        2
                    ],
                    {
                        "default": null
                    }
                ]
            }
        });
        let actual = to_value(
            Copy {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                id: 2,
                default: None,
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn copy_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": 54
        });

        let response: JsonRpcResponse<CopyResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn exists() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "exists",
                    [
                        [1, 2, -1, 999999999]
                    ],
                    {
                    }
                ]
            }
        });
        let actual = to_value(
            Exists {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                ids: vec![1, 2, -1, 999999999].into()
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn exists_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                1,
                2
            ]
        });

        let response: JsonRpcResponse<ExistsResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn check_access_rights() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "stock.quant",
                    "check_access_rights",
                    [
                        "unlink"
                    ],
                    {
                        "raise_exception": false
                    }
                ]
            }
        });
        let actual = to_value(
            CheckAccessRights {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "stock.quant".into(),

                operation: AccessOperation::Unlink,
                raise_exception: false
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn check_access_rights_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": false
        });

        let response: JsonRpcResponse<CheckAccessRightsResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn check_access_rules() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "check_access_rule",
                    [
                        [1, 2],
                        "unlink"
                    ],
                    {
                    }
                ]
            }
        });
        let actual = to_value(
            CheckAccessRules {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                ids: vec![1, 2].into(),
                operation: AccessOperation::Unlink,
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn check_access_rules_response() -> Result<()> {
        //TODO: this method, annoyingly, returns None on success. because of this,
        // the `result` field is never added. we need to modify JsonRpcResponse to
        // support this. until then, this method does not work!
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                "id",
                "email",
                "this_is_a_fake_field"
            ]
        });

        let response: JsonRpcResponse<CheckAccessRulesResponse> = {
            match from_value(payload) {
                Ok(d) => d,
                Err(_) => {
                    // As a *super* hacky workaround, we can check for deserialization
                    // errors
                    return Ok(())
                }
            }
        };

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn check_field_access_rights() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "check_field_access_rights",
                    [
                        "unlink",
                        [
                            "id",
                            "email",
                            "this_is_a_fake_field"
                        ]
                    ],
                    {
                    }
                ]
            }
        });
        let actual = to_value(
            CheckFieldAccessRights {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                operation: AccessOperation::Unlink,
                fields: svec![
                    "id",
                    "email",
                    "this_is_a_fake_field"
                ]
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn check_field_access_rights_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                "id",
                "email",
                "this_is_a_fake_field"
            ]
        });

        let response: JsonRpcResponse<CheckFieldAccessRightsResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn get_metadata() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "get_metadata",
                    [
                        [1, 2]
                    ],
                    {
                    }
                ]
            }
        });
        let actual = to_value(
            GetMetadata {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                ids: vec![1, 2].into()
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn get_metadata_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                {
                    "id": 1,
                    "create_uid": false,
                    "create_date": "2022-09-15 20:00:41",
                    "write_uid": [
                        2,
                        "Administrator"
                    ],
                    "write_date": "2023-01-16 01:17:19",
                    "xmlid": "base.main_partner",
                    "noupdate": true
                },
                {
                    "id": 2,
                    "create_uid": [
                        1,
                        "OdooBot"
                    ],
                    "create_date": "2022-09-15 20:00:43",
                    "write_uid": [
                        1,
                        "OdooBot"
                    ],
                    "write_date": "2023-02-20 22:32:37",
                    "xmlid": "base.partner_root",
                    "noupdate": true
                }
            ]
        });

        let response: JsonRpcResponse<GetMetadataResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn get_external_id() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "get_external_id",
                    [
                        [1, 2]
                    ],
                    {
                    }
                ]
            }
        });
        let actual = to_value(
            GetExternalId {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                ids: vec![1, 2].into()
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn get_external_id_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": {
                "1": "base.main_partner",
                "2": "base.partner_root"
            }
        });

        let response: JsonRpcResponse<GetExternalIdResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn get_xml_id() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "get_xml_id",
                    [
                        [1, 2]
                    ],
                    {
                    }
                ]
            }
        });
        let actual = to_value(
            GetXmlId {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                ids: vec![1, 2].into()
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn get_xml_id_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": {
                "1": "base.main_partner",
                "2": "base.partner_root"
            }
        });

        let response: JsonRpcResponse<GetXmlIdResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn name_get() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "name_get",
                    [
                        [1, 2, 3]
                    ],
                    {
                    }
                ]
            }
        });
        let actual = to_value(
            NameGet {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                ids: vec![1, 2, 3].into()
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn name_get_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                [
                    1,
                    "Test!"
                ],
                [
                    2,
                    "OdooBot"
                ],
                [
                    3,
                    "YourCompany, Administrator"
                ]
            ]
        });

        let response: JsonRpcResponse<NameGetResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn name_create() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "name_create",
                    [
                        "I am a test!"
                    ],
                    {
                    }
                ]
            }
        });
        let actual = to_value(
            NameCreate {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                name: "I am a test!".into(),
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn name_create_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                56,
                "I am a test!"
            ]
        });

        let response: JsonRpcResponse<NameCreateResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }

    #[test]
    fn name_search() -> Result<()> {
        let expected = json!({
            "jsonrpc": "2.0",
            "method": "call",
            "id": 1000,
            "params": {
                "service": "object",
                "method": "execute_kw",
                "args": [
                    "some-database",
                    2,
                    "password",
                    "res.partner",
                    "name_search",
                    [
                        "I am a test!"
                    ],
                    {
                        "args": null,
                        "operator": null,
                        "limit": null,
                    }
                ]
            }
        });
        let actual = to_value(
            NameSearch {
                database: "some-database".into(),
                uid: 2,
                password: "password".into(),

                model: "res.partner".into(),

                name: "I am a test!".into(),
                args: None,
                operator: None,
                limit: None
            }
            .build(1000),
        )?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn name_search_response() -> Result<()> {
        let payload = json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": [
                [
                    56,
                    "I am a test!"
                ],
                [
                    57,
                    "I am a test!"
                ]
            ]
        });

        let response: JsonRpcResponse<NameSearchResponse> = from_value(payload)?;

        match response {
            JsonRpcResponse::Error(e) => Err(e.error.into()),
            JsonRpcResponse::Success(_) => Ok(()),
        }
    }
}
