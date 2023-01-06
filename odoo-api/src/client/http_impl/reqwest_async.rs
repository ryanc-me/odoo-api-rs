use crate::client::error::{ReqwestAuthResult, ReqwestError, ReqwestResult};
use crate::client::{AuthState, Authed, NotAuthed, OdooClient, OdooRequest, RequestImpl};
use crate::jsonrpc::JsonRpcParams;
use reqwest::Client;
use serde::Serialize;
use std::fmt::Debug;

pub struct ReqwestAsync {
    client: Client,
}
impl RequestImpl for ReqwestAsync {
    type Error = ReqwestError;
}

impl OdooClient<NotAuthed, ReqwestAsync> {
    pub fn new_reqwest_async(url: &str) -> Result<Self, reqwest::Error> {
        let client = Client::builder().cookie_store(true).build()?;

        Ok(Self::new(url, ReqwestAsync { client }))
    }
}

impl<S> OdooClient<S, ReqwestAsync>
where
    S: AuthState,
{
    pub async fn authenticate(
        mut self,
        db: &str,
        login: &str,
        password: &str,
    ) -> ReqwestAuthResult<OdooClient<Authed, ReqwestAsync>> {
        let request = self.get_auth_request(db, login, password);
        let (response, session_id) = request.send_internal().await?;
        Ok(self.parse_auth_response(db, login, password, response, session_id)?)
    }
}

impl<'a, T> OdooRequest<'a, T, ReqwestAsync>
where
    T: JsonRpcParams + Debug + Serialize,
    T::Container<T>: Debug + Serialize,
{
    pub async fn send(self) -> ReqwestResult<T::Response> {
        Ok(self.send_internal().await?.0)
    }

    async fn send_internal(self) -> ReqwestResult<(T::Response, Option<String>)> {
        let request = self._impl.client.post(&self.url).json(&self.data);
        let response = request.send().await?;
        Ok((self.parse_response(&response.text().await?)?, None))
    }
}
