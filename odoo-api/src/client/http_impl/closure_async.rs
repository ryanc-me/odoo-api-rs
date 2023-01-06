use crate::client::error::{ClosureAuthResult, ClosureError, ClosureResult};
use crate::client::{AuthState, Authed, NotAuthed, OdooClient, OdooRequest, RequestImpl};
use crate::jsonrpc::JsonRpcParams;
use serde::Serialize;
use serde_json::{to_value, Value};
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

/// Convenience typedef. Use this as the return value for your async closure
pub type ClosureReturn = Pin<Box<dyn Future<Output = ClosureResult<(String, Option<String>)>>>>;
type Closure = Box<dyn Fn(String, Value, Option<String>) -> ClosureReturn>;

pub struct ClosureAsync {
    closure: Closure,
}
impl RequestImpl for ClosureAsync {
    type Error = ClosureError;
}

impl OdooClient<NotAuthed, ClosureAsync> {
    pub fn new_closure_async(
        url: &str,
        closure: impl 'static
            + Fn(
                String,
                Value,
                Option<String>,
            )
                -> Pin<Box<dyn Future<Output = ClosureResult<(String, Option<String>)>>>>,
    ) -> Self {
        Self::new(
            url,
            ClosureAsync {
                closure: Box::new(closure),
            },
        )
    }
}

impl<S> OdooClient<S, ClosureAsync>
where
    S: AuthState,
{
    pub async fn authenticate(
        mut self,
        db: &str,
        login: &str,
        password: &str,
    ) -> ClosureAuthResult<OdooClient<Authed, ClosureAsync>> {
        let request = self.get_auth_request(db, login, password);
        let (response, session_id) = request.send_internal().await?;
        Ok(self.parse_auth_response(db, login, password, response, session_id)?)
    }
}

impl<'a, T> OdooRequest<'a, T, ClosureAsync>
where
    T: JsonRpcParams + Debug + Serialize,
    T::Container<T>: Debug + Serialize,
{
    pub async fn send(self) -> ClosureResult<T::Response> {
        Ok(self.send_internal().await?.0)
    }

    async fn send_internal(self) -> ClosureResult<(T::Response, Option<String>)> {
        let data = to_value(&self.data)?;
        let (response, session_id) = (self._impl.closure)(
            self.url.clone(),
            data,
            self.session_id.map(|s| s.to_string()),
        )
        .await?;
        Ok((self.parse_response(&response)?, session_id))
    }
}
