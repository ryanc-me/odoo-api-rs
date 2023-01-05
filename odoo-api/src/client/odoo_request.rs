//! The [`OdooRequest`] type and associated bits

use super::RequestImpl;
use crate::jsonrpc::{JsonRpcParams, JsonRpcRequest, JsonRpcResponse};
use crate::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::from_str;
use std::fmt::Debug;

pub struct OdooRequest<'a, T, I>
where
    T: JsonRpcParams + Debug + Serialize,
    T::Container<T>: Debug + Serialize,
    I: RequestImpl,
{
    pub(crate) data: JsonRpcRequest<T>,
    pub(crate) url: String,
    pub(crate) session_id: Option<&'a str>,
    pub(crate) _impl: &'a I,
}

impl<'a, T, I> OdooRequest<'a, T, I>
where
    T: JsonRpcParams + Debug + Serialize,
    T::Container<T>: Debug + Serialize,
    I: RequestImpl,
{
    pub(crate) fn new(
        data: JsonRpcRequest<T>,
        url: String,
        session_id: Option<&'a str>,
        _impl: &'a I,
    ) -> Self {
        Self {
            data,
            url,
            session_id,
            _impl,
        }
    }

    pub(crate) fn parse_response<D: Debug + DeserializeOwned>(&self, data: &str) -> Result<D> {
        let response: JsonRpcResponse<D> = from_str(data)?;

        match response {
            JsonRpcResponse::Success(data) => Ok(data.result),
            JsonRpcResponse::Error(data) => Err(data.error.into()),
        }
    }
}
