//! The base Odoo API types
//!
//! This module contains raw types and impls for the Odoo API methods.
//!
//! As a crate user, you shouldn't need to interact with these directly. Instead, see [`crate::client`].
//!
//! <br />
//!
//! ## API Methods
//!
//! <br />
//!
//! <span style="font-size: 1.125rem; margin: 15px 0 5px 0;">[`object`](crate::service::object)</span>
//!
//! |<div style="width: 250px">Method</div>|<div style="width: 550px">Description</div>|<div style="width: 50px">Auth?</div>|
//! |-|-|-|
//! |[`execute`](object::Execute)|Call a business-logic method on an Odoo model (positional args)|**Yes**|
//! |[`execute_kw`](object::ExecuteKw)|Call a business-logic method on an Odoo model (positional & keyword args)|**Yes**|
//!
//! <br />
//!
//! <span style="font-size: 1.125rem; margin: 15px 0 5px 0;">[`orm`](crate::service::orm)</span>
//!
//! **TBC**
//!
//! |<div style="width: 250px">Method</div>|<div style="width: 550px">Description</div>|<div style="width: 50px">Auth?</div>|
//! |-|-|-|
//! |[`create`](orm::Create)|Create a new record (or set of records)|**Yes**|
//! |[`read`](orm::Read)|Read data from a record (or set of records)|**Yes**|
//! |[`read_group`](orm::ReadGroup)|Read some grouped data from a record (or set of records)|**Yes**|
//! |[`write`](orm::Write)|Write data to a record (or set of records)|**Yes**|
//! |[`unlink`](orm::Unlink)|Delete a record (or set of records)|**Yes**|
//! |[`search`](orm::Search)|Return the ids of records matching a domain|**Yes**|
//! |[`search_count`](orm::SearchCount)|Return the count of records matching a domain|**Yes**|
//! |[`search_read`](orm::SearchRead)|Perform a `search` and `read` in one call|**Yes**|
//! |[`copy`](orm::Copy)|Copy a record|**Yes**|
//! |[`exists`](orm::Exists)|Check if the record(s) exist in the Odoo database|**Yes**|
//! |[`check_access_rights`](orm::CheckAccessRights)|Check model access rights (according to `ir.model.access`)|**Yes**|
//! |[`check_access_rules`](orm::CheckAccessRules)|Check model access rules (according to `ir.rule`)|**Yes**|
//! |[`check_field_access_rights`](orm::CheckFieldAccessRights)|Check the user access rights on the given fields|**Yes**|
//! |[`get_metadata`](orm::GetExternalId)|Return some metadata about the given record(s)|**Yes**|
//! |[`get_external_id`](orm::GetMetadata)|Fetch the XMLID for the given record(s)|**Yes**|
//! |[`get_xml_id`](orm::GetXmlId)|Fetch the XMLID for the given record(s)|**Yes**|
//! |[`name_get`](orm::NameGet)|Fetch the `display_naame` for the given record(s)|**Yes**|
//! |[`name_create`](orm::NameCreate)|Create a new record, passing only the `name` field|**Yes**|
//! |[`name_search`](orm::NameSearch)|Search for records based on their `name` field|**Yes**|
//!
//! <br />
//!
//! <span style="font-size: 1.125rem; margin: 15px 0 5px 0;">[`common`](crate::service::common)</span>
//!
//! |<div style="width: 250px">Method</div>|<div style="width: 550px">Description</div>|<div style="width: 50px">Auth?</div>|
//! |-|-|-|
//! |[`common_login`](common::Login)|Check the user credentials and return the user ID|-|
//! |[`common_authenticate`](common::Authenticate)|Check the user credentials and return the user ID (web)|-|
//! |[`common_version`](common::Version)|Fetch detailed information about the Odoo version|-|
//! |[`common_about`](common::About)|Fetch basic information about the Odoo version|-|
//!
//! <br />
//!
//! <span style="font-size: 1.125rem; margin: 15px 0 5px 0;">[`db`](crate::service::db)</span>
//!
//! |<div style="width: 250px">Method</div>|<div style="width: 550px">Description</div>|<div style="width: 50px">Auth?</div>|
//! |-|-|-|
//! |[`db_create_database`](db::CreateDatabase)|Create and initialize a new database|-|
//! |[`db_duplicate_database`](db::DuplicateDatabase)|Duplicate a database|-|
//! |[`db_drop`](db::Drop)|Drop (delete) a database|-|
//! |[`db_dump`](db::Dump)|Dump (backup) a database, optionally including the filestore folder|-|
//! |[`db_restore`](db::Restore)|Upload and restore an Odoo dump to a new database|-|
//! |[`db_rename`](db::Rename)|Rename a database|-|
//! |[`db_change_admin_password`](db::ChangeAdminPassword)|Change the Odoo "master password"|-|
//! |[`db_migrate_database`](db::MigrateDatabases)|Perform a "database migration" (upgrade the `base` module)|-|
//! |[`db_exist`](db::DbExist)|Check if a database exists|-|
//! |[`db_list`](db::List)|List the databases currently available to Odoo|-|
//! |[`db_list_lang`](db::ListLang)|List the languages available to Odoo (ISO name + code)|-|
//! |[`db_list_countries`](db::ListCountries)|List the countries available to Odoo (ISO name + code)|-|
//! |[`db_server_version`](db::ServerVersion)|Return the server version|-|
//!
//! <br />
//!
//! <span style="font-size: 1.125rem; margin: 15px 0 5px 0;">[`web`](crate::service::web)</span>
//!
//! |<div style="width: 250px">Method</div>|<div style="width: 550px">Description</div>|<div style="width: 50px">Auth?</div>|
//! |-|-|-|
//! |[`web_session_authenticate`](web::SessionAuthenticate)|Docs TBC|-|
//!

pub mod common;
pub mod db;
pub mod object;
pub mod orm;
pub mod web;
