//! Async API functions, using `reqwest` as a backend

use super::{types};


pub mod common {
    //! The Odoo "common" service (blocking)

    pub use super::types::common::{
        login_blocking as login,
        authenticate_blocking as authenticate,
        version_blocking as version,
        about_blocking as about
    };
}

pub mod db {
    //! The Odoo "db" service (blocking)

    pub use super::types::db::{
        create_database_blocking as create_database,
        duplicate_database_blocking as duplicate_database,
        drop_blocking as drop,
        dump_blocking as dump,
        restore_blocking as restore,
        rename_blocking as rename,
        change_admin_password_blocking as change_admin_password,
        migrate_databases_blocking as migrate_databases,
        db_exist_blocking as db_exist,
        list_blocking as list,
        list_lang_blocking as list_lang,
        list_countries_blocking as list_countries,
    };
}

pub mod object {
    //! The Odoo "object" service (blocking)

    pub use super::types::object::{
        execute_blocking as execute,
        execute_kw_blocking as execute_kw,
    };
}