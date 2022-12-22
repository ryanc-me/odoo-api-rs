//! The Odoo "db" service (types only)

use odoo_api_macros::odoo_api_request;
use serde::{Deserialize, Serialize};

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L136-L142)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "create_database",
    "Create and initialize a new database"
)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct CreateDatabaseResponse {
    pub ok: bool,
}

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L144-L184)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "duplicate_database",
    "Duplicate a database"
)]
pub struct DuplicateDatabase {
    pub passwd: String,
    pub db_original_name: String,
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DuplicateDatabaseResponse {
    pub ok: bool,
}

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L212-L217)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "drop",
    "Drop (delete) a database"
)]
pub struct Drop {
    pub passwd: String,
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DropResponse {
    pub ok: bool,
}

/// Note that the data is returned a base64-encoded buffer.
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L212-L217)  
/// See also: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L219-L269)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "dump",
    "Dump (backup) a database, optionally including the filestore folder"
)]
pub struct Dump {
    pub passwd: String,
    pub db_name: String,
    pub format: crate::jsonrpc::types::db::DumpFormat,
}

/// The format for a database dump
#[derive(Debug, Serialize, Deserialize, PartialEq)]
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
pub struct DumpResponse {
    pub b64_bytes: String,
}

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L271-L284)  
/// See also: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L286-L335)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "restore",
    "Upload and restore an Odoo dump to a new database"
)]
pub struct Restore {
    pub passwd: String,
    pub b64_data: String,
    pub copy: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RestoreResponse {
    pub ok: bool,
}

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L337-L358)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "rename",
    "Rename a database"
)]
pub struct Rename {
    pub passwd: String,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RenameResponse {
    pub ok: bool,
}

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L360-L364)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "change_admin_password",
    "Change the Odoo \"master password\""
)]
pub struct ChangeAdminPassword {
    pub passwd: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChangeAdminPasswordResponse {
    pub ok: bool,
}

/// Note that this method doesn't actually perform any upgrades - instead, it
/// force-update the `base` module, which has the effect of triggering an update
/// on all Odoo modules that depend on `base` (which is all of them).
///
/// This method is probably used internally by Odoo's upgrade service, and likely
/// isn't useful on its own. If you need to upgrade a module, the [`execute`][crate::jsonrpc::types::object::execute]
/// is probably more suitable.
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L366-L372)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "migrate_databases",
    "Perform a \"database migration\" (upgrade the `base` module)"
)]
pub struct MigrateDatabases {
    pub passwd: String,
    pub databases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MigrateDatabasesResponse {
    pub ok: bool,
}

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L378-L386)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "db_exist",
    "Check if a database exists"
)]
pub struct DbExist {
    pub db_name: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DbExistResponse(pub bool);

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L439-L442)  
/// See also: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L388-L409)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "list",
    "List the databases currently available to Odoo"
)]
pub struct List {
    pub document: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ListResponse {
    pub databases: Vec<String>,
}

/// Note that this function is used by the database manager, in order to let the
/// user select which language should be used when creating a new database.
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L444-L445)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "list_lang",
    "List the languages available to Odoo (ISO name + code)"
)]
pub struct ListLang {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ListLangResponse {
    pub languages: Vec<ListLangResponseItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListLangResponseItem {
    pub code: String,
    pub name: String,
}

/// Note that this function is used by the database manager, in order to let the
/// user select which country should be used when creating a new database.
///
/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L447-L454)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "list_countries",
    "List the countries available to Odoo (ISO name + code)"
)]
pub struct ListCountries {
    pub passwd: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ListCountriesResponse {
    pub countries: Vec<ListLangResponseItem>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListCountriesResponseItem {
    pub code: String,
    pub name: String,
}

/// Docs TBC
///
/// Reference: [odoo/service/db.py](https://github.com/odoo/odoo/blob/b6e195ccb3a6c37b0d980af159e546bdc67b1e42/odoo/service/db.py#L456-L460)
#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request(
    "db", "server_version",
    "Return the server version"
)]
pub struct ServerVersion {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(transparent)]
pub struct ServerVersionResponse {
    pub version: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::jsonrpc::{JsonRpcResponseSuccess, JsonRpcVersion, OdooApiResponse, Result};
    use serde_json::{json, to_value};

    #[test]
    fn server_version() -> Result<()> {
        let expected_request = to_value(json!({
            "version": "2.0",
            "id": 1000,
            "method": "call",
            "params": {
                "service": "db",
                "method": "server_version",
                "args": []
            }
        }))?;
        let expected_response = to_value(json!({
            "jsonrpc": "2.0",
            "id": 1000,
            "result": "14.0+e"
        }))?;

        let request = super::server_version()?.to_json_value()?;

        let response = to_value(OdooApiResponse::<ServerVersion>::Success(
            JsonRpcResponseSuccess {
                jsonrpc: JsonRpcVersion::V2,
                id: 1000,
                result: ServerVersionResponse {
                    version: "14.0+e".into(),
                },
            },
        ))?;

        assert_eq!(request, expected_request);
        assert_eq!(response, expected_response);

        Ok(())
    }
}
