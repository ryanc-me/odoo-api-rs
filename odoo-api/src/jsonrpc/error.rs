use serde_json;
use thiserror::Error;
use super::JsonRpcError;

/// Convenience wrapper on the std `Result`
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

    #[cfg(any(feature="nonblocking", feature="blocking"))]
    #[error(transparent)]
    ReqwestError (#[from] reqwest::Error),

    /// The generic "Odoo Server Error"
    ///
    /// The majority of real-world errors will fall into this category. These
    /// error
    #[error("Odoo Server Error")]
    OdooServerError(JsonRpcError),

    #[error("404: Not Found")]
    OdooNotFoundError(JsonRpcError),

    #[error("Odoo Session Expired")]
    OdooSessionExpiredError(JsonRpcError),

    #[error("Odoo API Error")]
    OdooError(JsonRpcError),
}

#[derive(Debug)]
pub struct OdooApiError {

}