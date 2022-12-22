//! Blocking API functions, using `reqwest` as a backend

use super::types;

pub mod common {
    //! The Odoo "common" service (async)

    pub use super::types::common::{
        about_async::about_async as about, authenticate_async::authenticate_async as authenticate,
        login_async::login_async as login, version_async::version_async as version,
    };
}

pub mod db {
    //! The Odoo "db" service (async)

    pub use super::types::db::{
        change_admin_password_async::change_admin_password_async as change_admin_password,
        create_database_async::create_database_async as create_database,
        db_exist_async::db_exist_async as db_exist, drop_async::drop_async as drop,
        dump_async::dump_async as dump,
        duplicate_database_async::duplicate_database_async as duplicate_database,
        list_async::list_async as list,
        list_countries_async::list_countries_async as list_countries,
        list_lang_async::list_lang_async as list_lang,
        migrate_databases_async::migrate_databases_async as migrate_databases,
        rename_async::rename_async as rename, restore_async::restore_async as restore,
    };
}

pub mod object {
    //! The Odoo "object" service (async)

    pub use super::types::object::{
        execute_async::execute_async as execute, execute_kw_async::execute_kw_async as execute_kw,
    };
}
