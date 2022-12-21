use serde_json;
use thiserror::Error;

/// Convenience wrapper on std's `Result`
pub type Result<T> = ::std::result::Result<T, Error>;

/// An error returned by one of the Odoo API methods
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// A parsing error from the serde_json library
    ///
    /// This might be raised if the returned JSON data is invalid, or couldn't
    /// be parsed into the `XxxResponse` struct properly.
    #[error(transparent)]
    SerdeJsonError (#[from] serde_json::Error),

    /// The generic "Odoo Server Error"
    ///
    /// The majority of real-world errors will fall into this category. These
    /// error
    #[error("Odoo Server Error")]
    OdooServerError(),

    #[error("404: Not Found")]
    OdooNotFoundError(),

    #[error("Odoo Session Expired")]
    OdooSessionExpiredError(),

    #[error("Odoo API Error")]
    OdooError(),
}

#[derive(Debug)]
pub struct OdooApiError {

}