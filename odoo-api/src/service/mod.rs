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
//! |||
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
