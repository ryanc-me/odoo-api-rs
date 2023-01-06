use crate::jsonrpc::response::JsonRpcError;
use thiserror::Error;

/// An error during the response parsing phase
///
/// This error is used internally, and is typically parsed into either a
/// [`ClosureError`] or a [`ReqwestError`].
#[derive(Debug, Error)]
pub enum ParseResponseError {
    /// A parsing error from the serde_json library
    ///
    /// This might be raised if the returned JSON data is invalid, or couldn't
    /// be parsed into the `XxxResponse` struct properly.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    /// The Odoo API request was not successful
    ///
    /// See [`JsonRpcError`] for more details
    #[error("JSON-RPC Error")]
    JsonRpcError(#[from] JsonRpcError),
}

pub type ParseResponseResult<T> = std::result::Result<T, ParseResponseError>;

#[derive(Debug, Error)]
pub enum AuthenticationError {
    /// A parsing error from the serde_json library
    ///
    /// This might be raised if the returned JSON data is invalid, or couldn't
    /// be parsed into the `XxxResponse` struct properly.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    /// An error occured while parsing the `uid` field from the authenticate
    /// response
    #[error("UID Parser Error")]
    UidParseError(String),
}

pub type AuthenticationResult<T> = std::result::Result<T, AuthenticationError>;

/// An error sending a closure-based [`OdooRequest`](crate::client::OdooRequest)
///
///
#[derive(Debug, Error)]
pub enum ClosureError {
    /// An error occured inside the custom closure
    ///
    /// We include a blanket from Box<dyn Error> here because the concrete error
    /// type cannot be known here (i.e., only the crate *consumer* will know the
    /// type). This allows `fallible()?` to correctly return the ClosureError type
    #[error(transparent)]
    ClosureError(#[from] Box<dyn std::error::Error>),

    /// A parsing error from the serde_json library
    ///
    /// This might be raised if the returned JSON data is invalid, or couldn't
    /// be parsed into the `XxxResponse` struct properly.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    /// The Odoo API request was not successful
    ///
    /// See [`JsonRpcError`] for more details
    #[error("JSON-RPC Error")]
    JsonRpcError(#[from] JsonRpcError),
}

// This is nicer than having a `ParseError` variant on the `ClosureError` struct
// (which would duplicate these fields anyways)
impl From<ParseResponseError> for ClosureError {
    fn from(value: ParseResponseError) -> Self {
        match value {
            ParseResponseError::JsonRpcError(err) => Self::JsonRpcError(err),
            ParseResponseError::SerdeJsonError(err) => Self::SerdeJsonError(err),
        }
    }
}

pub type ClosureResult<T> = std::result::Result<T, ClosureError>;

/// An error during the `authenticate()` call
#[derive(Debug, Error)]
pub enum ClosureAuthError {
    /// An error occured during the serialization, sending, receiving, or deserialization
    /// of the request
    #[error(transparent)]
    ClosureError(#[from] ClosureError),

    /// An error occured while parsing the `uid` field from the authenticate
    /// response
    #[error("UID Parser Error")]
    UidParseError(String),
}

// As with `From<ParseResponseError>`, we'd like to avoid having duplicate error fields
impl From<AuthenticationError> for ClosureAuthError {
    fn from(value: AuthenticationError) -> Self {
        match value {
            AuthenticationError::SerdeJsonError(err) => {
                Self::ClosureError(ClosureError::SerdeJsonError(err))
            }
            AuthenticationError::UidParseError(err) => Self::UidParseError(err),
        }
    }
}

pub type ClosureAuthResult<T> = std::result::Result<T, ClosureAuthError>;

#[derive(Debug, Error)]
pub enum ReqwestError {
    /// An error from the [`reqwest`] library
    ///
    /// See [`reqwest::Error`] for more information.
    #[cfg(any(feature = "async", feature = "blocking"))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// A parsing error from the serde_json library
    ///
    /// This might be raised if the returned JSON data is invalid, or couldn't
    /// be parsed into the `XxxResponse` struct properly.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    /// The Odoo API request was not successful
    ///
    /// See [`JsonRpcError`] for more details
    #[error("JSON-RPC Error")]
    JsonRpcError(#[from] JsonRpcError),
}

impl From<ParseResponseError> for ReqwestError {
    fn from(value: ParseResponseError) -> Self {
        match value {
            ParseResponseError::JsonRpcError(err) => Self::JsonRpcError(err),
            ParseResponseError::SerdeJsonError(err) => Self::SerdeJsonError(err),
        }
    }
}

pub type ReqwestResult<T> = std::result::Result<T, ReqwestError>;

#[derive(Debug, Error)]
pub enum ReqwestAuthError {
    #[error(transparent)]
    ReqwestError(#[from] ReqwestError),

    /// An error occured while parsing the `uid` field from the authenticate
    /// response
    #[error("UID Parser Error")]
    UidParseError(String),
}

// As with `From<ParseResponseError>`, we'd like to avoid having duplicate error fields
impl From<AuthenticationError> for ReqwestAuthError {
    fn from(value: AuthenticationError) -> Self {
        match value {
            AuthenticationError::SerdeJsonError(err) => {
                Self::ReqwestError(ReqwestError::SerdeJsonError(err))
            }
            AuthenticationError::UidParseError(err) => Self::UidParseError(err),
        }
    }
}

pub type ReqwestAuthResult<T> = std::result::Result<T, ReqwestAuthError>;

#[derive(Debug, Error)]
pub enum Error {
    /// An error occured inside the custom closure
    ///
    /// We include a blanket from Box<dyn Error> here because the concrete error
    /// type cannot be known here (i.e., only the crate *consumer* will know the
    /// type). This allows `fallible()?` to correctly return the ClosureError type
    #[error(transparent)]
    ClosureError(#[from] Box<dyn std::error::Error>),

    /// An error from the [`reqwest`] library
    ///
    /// See [`reqwest::Error`] for more information.
    #[cfg(any(feature = "async", feature = "blocking"))]
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),

    /// A parsing error from the serde_json library
    ///
    /// This might be raised if the returned JSON data is invalid, or couldn't
    /// be parsed into the `XxxResponse` struct properly.
    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    /// The Odoo API request was not successful
    ///
    /// See [`JsonRpcError`] for more details
    #[error("JSON-RPC Error")]
    JsonRpcError(#[from] JsonRpcError),

    /// An error occured while parsing the `uid` field from the authenticate
    /// response
    #[error("UID Parser Error")]
    UidParseError(String),
}

// This is nicer than having a `ParseError` variant on the `ClosureError` struct
// (which would duplicate these fields anyways)
impl From<ParseResponseError> for Error {
    fn from(value: ParseResponseError) -> Self {
        match value {
            ParseResponseError::JsonRpcError(err) => Self::JsonRpcError(err),
            ParseResponseError::SerdeJsonError(err) => Self::SerdeJsonError(err),
        }
    }
}

// As with `From<ParseResponseError>`, we'd like to avoid having duplicate error fields
impl From<AuthenticationError> for Error {
    fn from(value: AuthenticationError) -> Self {
        match value {
            AuthenticationError::SerdeJsonError(err) => Self::SerdeJsonError(err),
            AuthenticationError::UidParseError(err) => Self::UidParseError(err),
        }
    }
}

impl From<ClosureError> for Error {
    fn from(value: ClosureError) -> Self {
        match value {
            ClosureError::ClosureError(err) => Self::ClosureError(err),
            ClosureError::JsonRpcError(err) => Self::JsonRpcError(err),
            ClosureError::SerdeJsonError(err) => Self::SerdeJsonError(err),
        }
    }
}

impl From<ClosureAuthError> for Error {
    fn from(value: ClosureAuthError) -> Self {
        match value {
            ClosureAuthError::ClosureError(err) => err.into(),
            ClosureAuthError::UidParseError(err) => Self::UidParseError(err),
        }
    }
}

impl From<ReqwestError> for Error {
    fn from(value: ReqwestError) -> Self {
        match value {
            ReqwestError::ReqwestError(err) => Self::ReqwestError(err),
            ReqwestError::JsonRpcError(err) => Self::JsonRpcError(err),
            ReqwestError::SerdeJsonError(err) => Self::SerdeJsonError(err),
        }
    }
}

impl From<ReqwestAuthError> for Error {
    fn from(value: ReqwestAuthError) -> Self {
        match value {
            ReqwestAuthError::ReqwestError(err) => err.into(),
            ReqwestAuthError::UidParseError(err) => Self::UidParseError(err),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;
