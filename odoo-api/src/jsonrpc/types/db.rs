//! The Odoo "db" service (types only)

use serde::{Serialize, Deserialize};
use odoo_api_macros::odoo_api_request;


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "create_database")]
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


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "duplicate_database")]
pub struct DuplicateDatabase {
    pub passwd: String,
    pub db_original_name: String,
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DuplicateDatabaseResponse {
    pub ok: bool,
}


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "drop")]
pub struct Drop {
    pub passwd: String,
    pub db_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DropResponse {
    pub ok: bool,
}


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "dump")]
pub struct Dump {
    pub passwd: String,
    pub db_name: String,
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DumpResponse {
    pub b64_bytes: String,
}


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "restore")]
pub struct Restore {
    pub passwd: String,
    pub b64_data: String,
    pub copy: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RestoreResponse {
    pub ok: bool,
}


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "rename")]
pub struct Rename {
    pub passwd: String,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct RenameResponse {
    pub ok: bool,
}


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "change_admin_password")]
pub struct ChangeAdminPassword {
    pub passwd: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ChangeAdminPasswordResponse {
    pub ok: bool,
}


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "migrate_databases")]
pub struct MigrateDatabases {
    pub passwd: String,
    pub databases: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct MigrateDatabasesResponse {
    pub ok: bool,
}


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "db_exist")]
pub struct DbExist {
    pub db_name: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DbExistResponse (
    pub bool,
);


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "list")]
pub struct List {
    pub document: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListResponse (
    pub Vec<String>
);


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "list_lang")]
pub struct ListLang {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListLangResponse (
    Vec<ListLangResponseItem>
);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListLangResponseItem {
    pub code: String,
    pub name: String,
}


#[derive(Debug, Deserialize, PartialEq)]
#[odoo_api_request("db", "list_countries")]
pub struct ListCountries {
    pub passwd: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListCountriesResponse (
    Vec<ListLangResponseItem>
);

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ListCountriesResponseItem {
    pub code: String,
    pub name: String,
}
