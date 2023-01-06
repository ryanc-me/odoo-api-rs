use crate::client::error::{ReqwestAuthResult, ReqwestError, ReqwestResult};
use crate::client::{AuthState, Authed, NotAuthed, OdooClient, OdooRequest, RequestImpl};
use crate::jsonrpc::JsonRpcParams;
use reqwest::blocking::Client;
use serde::Serialize;
use std::fmt::Debug;

pub struct ReqwestBlocking {
    client: Client,
}
impl RequestImpl for ReqwestBlocking {
    type Error = ReqwestError;
}

impl OdooClient<NotAuthed, ReqwestBlocking> {
    pub fn new_reqwest_blocking(url: &str) -> Result<Self, reqwest::Error> {
        let client = Client::builder().cookie_store(true).build()?;

        Ok(Self::new(url, ReqwestBlocking { client }))
    }
}

impl<S> OdooClient<S, ReqwestBlocking>
where
    S: AuthState,
{
    pub fn authenticate(
        mut self,
        db: &str,
        login: &str,
        password: &str,
    ) -> ReqwestAuthResult<OdooClient<Authed, ReqwestBlocking>> {
        let request = self.get_auth_request(db, login, password);
        let (response, session_id) = request.send_internal()?;
        Ok(self.parse_auth_response(db, login, password, response, session_id)?)
    }
}

impl<'a, T> OdooRequest<'a, T, ReqwestBlocking>
where
    T: JsonRpcParams + Debug + Serialize,
    T::Container<T>: Debug + Serialize,
{
    pub fn send(self) -> ReqwestResult<T::Response> {
        Ok(self.send_internal()?.0)
    }

    fn send_internal(self) -> ReqwestResult<(T::Response, Option<String>)> {
        let request = self._impl.client.post(&self.url).json(&self.data);
        let response = request.send()?;
        Ok((self.parse_response(&response.text()?)?, None))
    }
}
