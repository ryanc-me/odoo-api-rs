//! The Odoo "db" service (JSON-RPC)
//!
//! This service handles database-management related methods (like create, drop, etc)
//!
//! Note that you will see some methods that require a `passwd` argument. This is **not**
//! the Odoo user password (database-level). Instead, it's the Odoo server-level
//! "master password", which can be found in the Odoo `.conf` file as the `admin_passwd` key.

use crate as odoo_api;
use crate::jsonrpc::OdooApiMethod;
use odoo_api_macros::odoo_api;
use serde::de::Visitor;
use serde::ser::SerializeTuple;
use serde::{Deserialize, Serialize};
use serde_tuple::Serialize_tuple;

/// Create and initialize a new database
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L136-L142)
#[odoo_api(
    service = "db",
    method = "create_database",
    name = "db_create_database",
    auth = false
)]
#[derive(Debug, Serialize_tuple)]
pub struct CreateDatabase {
    pub passwd: String,
    pub db_name: String,
    pub demo: bool,
    pub lang: String,
    pub user_password: String,
    pub login: String,
    pub country_code: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CreateDatabaseResponse {
    pub ok: bool,
}

/// Duplicate a database
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L144-L184)
#[odoo_api(
    service = "db",
    method = "duplicate_database",
    name = "db_duplicate_database",
    auth = false
)]
#[derive(Debug, Serialize_tuple)]
pub struct DuplicateDatabase {
    pub passwd: String,
    pub db_original_name: String,
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DuplicateDatabaseResponse {
    pub ok: bool,
}

/// Drop (delete) a database
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L212-L217)
#[odoo_api(service = "db", method = "drop", name = "db_drop", auth = false)]
#[derive(Debug, Serialize_tuple)]
pub struct Drop {
    pub passwd: String,
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DropResponse {
    pub ok: bool,
}

/// Dump (backup) a database, optionally including the filestore folder
///
/// Note that the data is returned a base64-encoded buffer.
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L212-L217)  
/// See also: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L219-L269)
#[odoo_api(service = "db", method = "dump", name = "db_dump", auth = false)]
#[derive(Debug, Serialize_tuple)]
pub struct Dump {
    pub passwd: String,
    pub db_name: String,
    pub format: crate::service::db::DumpFormat,
}

/// The format for a database dump
#[derive(Debug, Serialize, Deserialize)]
pub enum DumpFormat {
    /// Output a zipfile containing the SQL dump in "plain" format, manifest, and filestore
    ///
    /// Note that with this mode, the database is dumped to a Python
    /// NamedTemporaryFile first, then to the out stream - this means that
    /// the backup takes longer, and probably involves some filesystem writes.
    ///
    /// Also note that the SQL format is "plain"; that is, it's a text file
    /// containing SQL statements. This style of database dump is slightly less
    /// flexible when importing (e.g., you cannot choose to exclude some
    /// tables during import).
    ///
    /// See the [Postgres `pg_dump` docs](https://www.postgresql.org/docs/current/app-pgdump.html) for more info on "plain" dumps (`-F` option).
    #[serde(rename = "zip")]
    Zip,

    /// Output a `.dump` file containing the SQL dump in "custom" format
    ///
    /// This style of database dump is more flexible on the import side (e.g.,
    /// you can choose to exclude some tables from the import), but does not
    /// include the filestore.
    ///
    /// See the [Postgres `pg_dump` docs](https://www.postgresql.org/docs/current/app-pgdump.html) for more info on "custom" dumps (`-F` option).
    #[serde(rename = "dump")]
    Dump,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct DumpResponse {
    pub b64_bytes: String,
}

/// Upload and restore an Odoo dump to a new database
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L271-L284)  
/// See also: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L286-L335)
#[odoo_api(service = "db", method = "restore", name = "db_restore", auth = false)]
#[derive(Debug, Serialize_tuple)]
pub struct Restore {
    pub passwd: String,
    pub b64_data: String,
    pub restore_type: RestoreType,
}

/// The type of database restore
#[derive(Debug)]
pub enum RestoreType {
    /// Restore as a "copy"
    ///
    /// In this case, the database UUID is automatically updated to prevent
    /// conflicts.
    ///
    /// This is typically used when restoring a database for testing.
    Copy,

    /// Restore as a "move"
    ///
    /// In this case, the database UUID is **not** updated, and the database
    /// is restored as-is.
    ///
    /// This is typically used when restoring a database to a new hosting environment.
    Move,
}

// As far as I can tell, there isn't an easy way to serialize/deserialize
// a two-variant enum to/from a boolean, so we need to implement those manually.
// note that Deserialize isn't strictly necessary, but I'll include it for
// completeness.
impl Serialize for RestoreType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_bool(match self {
            Self::Copy => true,
            Self::Move => false,
        })
    }
}
struct RestoreTypeVisitor;
impl<'de> Visitor<'de> for RestoreTypeVisitor {
    type Value = bool;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a boolean (`true` or `false`)")
    }

    fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(v)
    }
}
impl<'de> Deserialize<'de> for RestoreType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let b = deserializer.deserialize_bool(RestoreTypeVisitor)?;

        Ok(match b {
            true => Self::Copy,
            false => Self::Move,
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct RestoreResponse {
    pub ok: bool,
}

/// Rename a database
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L337-L358)
#[odoo_api(service = "db", method = "rename", name = "db_rename", auth = false)]
#[derive(Debug, Serialize_tuple)]
pub struct Rename {
    pub passwd: String,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RenameResponse {
    pub ok: bool,
}

/// Change the Odoo "master password"
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L360-L364)
#[odoo_api(
    service = "db",
    method = "change_admin_password",
    name = "db_change_admin_password",
    auth = false
)]
#[derive(Debug, Serialize_tuple)]
pub struct ChangeAdminPassword {
    pub passwd: String,
    pub new_passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChangeAdminPasswordResponse {
    pub ok: bool,
}

/// Perform a "database migration" (upgrade the `base` module)
///
/// Note that this method doesn't actually perform any upgrades - instead, it
/// force-update the `base` module, which has the effect of triggering an update
/// on all Odoo modules that depend on `base` (which is all of them).
///
/// This method is probably used internally by Odoo's upgrade service, and likely
/// isn't useful on its own. If you need to upgrade a module, the [`Execute`][crate::service::object::Execute]
/// is probably more suitable.
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L366-L372)
#[odoo_api(
    service = "db",
    method = "migrate_databases",
    name = "db_migrate_databases",
    auth = false
)]
#[derive(Debug, Serialize_tuple)]
pub struct MigrateDatabases {
    pub passwd: String,
    pub databases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct MigrateDatabasesResponse {
    pub ok: bool,
}

/// Check if a database exists
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L378-L386)
#[odoo_api(service = "db", method = "db_exist", auth = false)]
#[derive(Debug, Serialize_tuple)]
pub struct DbExist {
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DbExistResponse {
    pub exists: bool,
}

/// List the databases currently available to Odoo
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L439-L442)  
/// See also: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L388-L409)
#[odoo_api(service = "db", method = "list", name = "db_list", auth = false)]
#[derive(Debug, Serialize_tuple)]
pub struct List {
    /// This argument isn't currently used and has no effect on the output
    pub document: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListResponse {
    pub databases: Vec<String>,
}

/// List the languages available to Odoo (ISO name + code)
///
/// Note that this function is used by the database manager, in order to let the
/// user select which language should be used when creating a new database.
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L444-L445)
#[odoo_api(
    service = "db",
    method = "list_lang",
    name = "db_list_lang",
    auth = false
)]
#[derive(Debug)]
pub struct ListLang {}

// ListLang has no fields, but needs to output in JSON: `[]`
impl Serialize for ListLang {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let state = serializer.serialize_tuple(0)?;
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListLangResponse {
    pub languages: Vec<ListLangResponseItem>,
}

#[derive(Debug, Serialize_tuple, Deserialize, PartialEq)]
pub struct ListLangResponseItem {
    pub code: String,
    pub name: String,
}

/// List the countries available to Odoo (ISO name + code)
///
/// Note that this function is used by the database manager, in order to let the
/// user select which country should be used when creating a new database.
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L447-L454)
#[odoo_api(
    service = "db",
    method = "list_countries",
    name = "db_list_countries",
    auth = false
)]
#[derive(Debug, Serialize_tuple)]
pub struct ListCountries {
    pub passwd: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ListCountriesResponse {
    pub countries: Vec<ListLangResponseItem>,
}

#[derive(Debug, Serialize_tuple, Deserialize)]
pub struct ListCountriesResponseItem {
    pub code: String,
    pub name: String,
}

/// Return the server version
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L456-L460)
#[odoo_api(
    service = "db",
    method = "server_version",
    name = "db_server_version",
    auth = false
)]
#[derive(Debug)]
pub struct ServerVersion {}

// ServerVersion has no fields, but needs to output in JSON: `[]`
impl Serialize for ServerVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let state = serializer.serialize_tuple(0)?;
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ServerVersionResponse {
    pub version: String,
}
