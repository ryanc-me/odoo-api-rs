//! Async API functions, using `reqwest` as a backend

use super::types;

pub mod common {
    //! The Odoo "common" service (blocking)

    pub use super::types::common::{
        about_blocking::about_blocking as about,
        authenticate_blocking::authenticate_blocking as authenticate,
        login_blocking::login_blocking as login, version_blocking::version_blocking as version,
    };
}

pub mod db {
    //! The Odoo "db" service (blocking)

    pub use super::types::db::{
        change_admin_password_blocking::change_admin_password_blocking as change_admin_password,
        create_database_blocking::create_database_blocking as create_database,
        db_exist_blocking::db_exist_blocking as db_exist, drop_blocking::drop_blocking as drop,
        dump_blocking::dump_blocking as dump,
        duplicate_database_blocking::duplicate_database_blocking as duplicate_database,
        list_blocking::list_blocking as list,
        list_countries_blocking::list_countries_blocking as list_countries,
        list_lang_blocking::list_lang_blocking as list_lang,
        migrate_databases_blocking::migrate_databases_blocking as migrate_databases,
        rename_blocking::rename_blocking as rename, restore_blocking::restore_blocking as restore,
    };
}

pub mod object {
    //! The Odoo "object" service (blocking)

    pub use super::types::object::{
        execute_blocking::execute_blocking as execute,
        execute_kw_blocking::execute_kw_blocking as execute_kw,
    };
}
