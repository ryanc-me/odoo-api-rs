use super::JsonRpcError;
use serde_json;
use thiserror::Error;

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
    SerdeJsonError(#[from] serde_json::Error),

    #[cfg(any(feature = "nonblocking", feature = "blocking"))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// The generic "Odoo Server Error"
    ///
    /// The majority of real-world errors will fall into this category. These
    /// error
    #[error("Odoo Server Error")]
    OdooServerError(JsonRpcError),

    /// An Odoo "not found" error
    ///
    /// This might be thrown if the wrong service/method were specified (which
    /// should not be possible with this library)
    #[error("404: Not Found")]
    OdooNotFoundError(JsonRpcError),

    /// An Odoo session-expired error
    ///
    /// This librarry doesn't use sessions, so this error should not be possible
    #[error("Odoo Session Expired")]
    OdooSessionExpiredError(JsonRpcError),

    /// A generic API error
    ///
    /// Typically all JSON-RPC errors will fall into one of the error types above.
    /// However, if the server is misconfigured, or there is a major issue in the
    /// request routing logic, the generic `OdooError` response might be returned.
    #[error("Odoo API Error")]
    OdooError(JsonRpcError),
}
