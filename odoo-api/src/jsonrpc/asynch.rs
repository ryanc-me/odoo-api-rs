//! Blocking API functions, using `reqwest` as a backend

use super::{types};


pub mod common {
    //! The Odoo "common" service (async)

    pub use super::types::common::{
        login_async::login_async as login,
        authenticate_async::authenticate_async as authenticate,
        version_async::version_async as version,
        about_async::about_async as about
    };
}

pub mod db {
    //! The Odoo "db" service (async)

    pub use super::types::db::{
        create_database_async::create_database_async as create_database,
        duplicate_database_async::duplicate_database_async as duplicate_database,
        drop_async::drop_async as drop,
        dump_async::dump_async as dump,
        restore_async::restore_async as restore,
        rename_async::rename_async as rename,
        change_admin_password_async::change_admin_password_async as change_admin_password,
        migrate_databases_async::migrate_databases_async as migrate_databases,
        db_exist_async::db_exist_async as db_exist,
        list_async::list_async as list,
        list_lang_async::list_lang_async as list_lang,
        list_countries_async::list_countries_async as list_countries,
    };
}

pub mod object {
    //! The Odoo "object" service (async)

    pub use super::types::object::{
        execute_async::execute_async as execute,
        execute_kw_async::execute_kw_async as execute_kw,
    };
}