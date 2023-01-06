use crate::client::error::{ClosureAuthResult, ClosureError, ClosureResult};
use crate::client::{AuthState, Authed, NotAuthed, OdooClient, OdooRequest, RequestImpl};
use crate::jsonrpc::JsonRpcParams;
use serde::Serialize;
use serde_json::{to_value, Value};
use std::fmt::Debug;

/// Convenience typedef. Use this as the return value for your blocking closure
pub type ClosureReturn = ClosureResult<(String, Option<String>)>;
type Closure = Box<dyn Fn(&str, Value, Option<&str>) -> ClosureReturn>;

pub struct ClosureBlocking {
    closure: Closure,
}
impl RequestImpl for ClosureBlocking {
    type Error = ClosureError;
}

impl OdooClient<NotAuthed, ClosureBlocking> {
    pub fn new_closure_blocking<
        F: Fn(&str, Value, Option<&str>) -> ClosureResult<(String, Option<String>)> + 'static,
    >(
        url: &str,
        closure: F,
    ) -> Self {
        Self::new(
            url,
            ClosureBlocking {
                closure: Box::new(closure),
            },
        )
    }
}

impl<S> OdooClient<S, ClosureBlocking>
where
    S: AuthState,
{
    pub fn authenticate(
        mut self,
        db: &str,
        login: &str,
        password: &str,
    ) -> ClosureAuthResult<OdooClient<Authed, ClosureBlocking>> {
        let request = self.get_auth_request(db, login, password);
        let (response, session_id) = request.send_internal()?;
        Ok(self.parse_auth_response(db, login, password, response, session_id)?)
    }
}

impl<'a, T> OdooRequest<'a, T, ClosureBlocking>
where
    T: JsonRpcParams + Debug + Serialize,
    T::Container<T>: Debug + Serialize,
{
    pub fn send(self) -> ClosureResult<T::Response> {
        Ok(self.send_internal()?.0)
    }

    fn send_internal(self) -> ClosureResult<(T::Response, Option<String>)> {
        let data = to_value(&self.data)?;
        let (response, session_id) = self._impl.closure.as_ref()(&self.url, data, self.session_id)?;
        Ok((self.parse_response(&response)?, session_id))
    }
}
