//! The user-facing API types
//!
//! This module provides a user-facing API for Odoo JSON-RPC methods.
//!
//! ##

pub use client::{OdooClient, AuthState, Authed, NotAuthed, RequestImpl};
pub use request::{OdooRequest};
pub use closure_blocking::ClosureResult as BlockingClosureResult;
pub use closure_async::ClosureResult as AsyncClosureResult;

#[allow(clippy::module_inception)]
mod client {
    //! Internal module to make the `client.rs` file more readable

    use std::fmt::{Debug};
    use serde::{Serialize};
    use serde_json::{from_str, to_string};
    use crate::{Result};
    use crate::jsonrpc::{OdooId, JsonRpcParams, OdooWebMethod};
    use crate::service::web::{SessionAuthenticate, SessionAuthenticateResponse};
    use super::{OdooRequest};

    /// The "authentication" state of a client object
    ///
    /// This is used to allow API methods to require authentication, e.g., if they
    /// require some piece of auth data (e.g. database, login/uid, etc).
    pub trait AuthState {
        /// Get the current stored `session_id`, if available
        fn get_session_id(&self) -> Option<&str>;
    }

    /// Implemented by "authenticated" clients
    pub struct Authed {
        pub(crate) database: String,
        pub(crate) login: String,
        pub(crate) uid: OdooId,
        pub(crate) password: String,
        pub(crate) session_id: Option<String>,
    }
    impl AuthState for Authed {
        fn get_session_id(&self) -> Option<&str> {
            self.session_id.as_deref()
        }
    }

    /// Implemented by "non-authenticated" clients
    pub struct NotAuthed {}
    impl AuthState for NotAuthed {
        fn get_session_id(&self) -> Option<&str> {
            None
        }
    }

    /// The "request implementation" for a client
    ///
    /// This is used to allow different `client.authenticate()` and
    /// `request.send()` impls based on the chosen request provider.
    pub trait RequestImpl {}

    /// An Odoo API client
    ///
    /// This is the main public interface for the `odoo-api` crate. It provides
    /// methods to authenticate with an Odoo instance, and to call JSON-RPC methods
    /// (`execute`, `create_database`, etc), "Web" methods (`/web/session/authenticate`, etc)
    /// and ORM methods (`read_group`, `create`, etc).
    ///
    /// ## Usage:
    /// ```no_run
    /// async fn test {
    ///     let url = "https://demo.odoo.com";
    ///     let client = OdooClient::new_request_async(url)
    ///         .authenticate(
    ///             "test-database",
    ///             "admin",
    ///             "password"
    ///         ).await?;
    ///
    ///     let user_ids = client.execute(
    ///         "res.users",
    ///         "search",
    ///         jvec![
    ///             []
    ///         ]
    ///     ).send().await?;
    /// 
    ///     println!("Found user IDs: {:?}", user_ids.data);
    /// }
    /// ```
    pub struct OdooClient<S, I>
    where
        S: AuthState,
        I: RequestImpl,
    {
        pub(crate) url: String,
        pub(crate) url_jsonrpc: String,

        pub(crate) auth: S,
        pub(crate) _impl: I
    }

    // Base client methods
    impl<S, I> OdooClient<S, I>
    where
        S: AuthState,
        I: RequestImpl
    {
        /// Validate and parse URLs
        /// 
        /// We cache the "/jsonrpc" endpoint because that's used across all of
        /// the JSON-RPC methods. We also store the bare URL, because that's
        /// used for "Web" methods
        pub(crate) fn build_urls(url: &str) -> (String, String) {
            let url = url.to_string();
            let url_jsonrpc = format!("{}/jsonrpc", url);

            (url, url_jsonrpc)
        }

        /// Build the data `T` into a request for the fully-qualified endpoint `url`
        ///
        /// This returns an [`OdooRequest`] typed to the Clients (`self`s) [`RequestImpl`],
        /// and to its auth state. The returned request is bound by lifetime `'a` to the client.
        /// The URL is converted into a full String, so no lifetimes apply there.
        pub(crate) fn build_request<'a, T>(&'a self, data: T, url: &str) -> OdooRequest<'a, T, I>
        where
            T: JsonRpcParams + Debug,
            T::Container<T>: Debug + Serialize,
            S: AuthState
        {
            OdooRequest::new(
                data.build(),
                url.into(),
                self.session_id(),
                &self._impl,
            )
        }

        /// Helper method to perform the 1st stage of the authentication request
        ///
        /// Implementors of [`RequestImpl`] will use this method to build an
        /// [`OdooRequest`], which they will then send using their own `send()` method.
        /// 
        /// This is necessary because each `RequestImpl` has its own `send()` signature
        /// (i.e., some are `fn send()`, some are `async fn send()`).
        pub(crate) fn get_auth_request(&self, db: &str, login: &str, password: &str) -> OdooRequest<SessionAuthenticate, I> {
            let authenticate = crate::service::web::SessionAuthenticate {
                db: db.into(),
                login: login.into(),
                password: password.into(),
            };
            let url_frag = authenticate.describe();

            self.build_request(
                authenticate,
                &format!("{}{}", &self.url, url_frag)
            )
        }

        /// Helper method to perform the 2nd stage of the authentication request
        ///
        /// At this point, the [`OdooRequest`] has been sent by the [`RequestImpl`],
        /// and the response data has been fetched and parsed.
        /// 
        /// This method extracts the `uid` and `session_id` from the resulting request,
        /// and returns an `OdooClient<Authed, I>`, e.g., an "authenticated" client.
        pub(crate) fn parse_auth_response(self, db: &str, login: &str, password: &str, response: SessionAuthenticateResponse, session_id: Option<String>) -> Result<OdooClient<Authed, I>> {
            let uid = response.data.get("uid").ok_or("Failed to parse UID from /web/session/authenticate call")?;
            //TODO: this is a bit awkward..
            let uid = from_str(&to_string(uid)?)?;
            let auth = Authed {
                database: db.into(),
                uid,
                login: login.into(),
                password: password.into(),
                session_id
            };

            Ok(OdooClient {
                url: self.url,
                url_jsonrpc: self.url_jsonrpc,
                auth,
                _impl: self._impl
            })
        }

        pub fn session_id(&self) -> Option<&str> {
            self.auth.get_session_id()
        }
    }

    /// Methods for non-authenticated clients
    impl<I> OdooClient<NotAuthed, I>
    where
        I: RequestImpl
    {
        /// Helper method to build a new client
        /// 
        /// This isn't exposed via the public API - instead, users will call
        /// one of the impl-specific `new_xx()` functions, like:
        ///  - OdooClient::new_request_blocking()
        ///  - OdooClient::new_request_async()
        ///  - OdooClient::new_closure_blocking()
        ///  - OdooClient::new_closure_async()
        pub(crate) fn new(url: &str, _impl: I) -> Self {
            let (url, url_jsonrpc) = Self::build_urls(url);
            Self {
                url,
                url_jsonrpc,
                auth: NotAuthed {},
                _impl
            }
        }
    }
}

mod request {
    use std::fmt::{Debug};
    use serde::{Serialize};
    use serde::de::{DeserializeOwned};
    use serde_json::{from_str};
    use crate::{Result};
    use crate::jsonrpc::{JsonRpcRequest, JsonRpcResponse, JsonRpcParams};
    use super::{RequestImpl};

    pub struct OdooRequest<'a, T, I>
    where
        T: JsonRpcParams + Debug + Serialize,
        T::Container<T>: Debug + Serialize,
        I: RequestImpl,
    {
        pub(crate) data: JsonRpcRequest<T>,
        pub(crate) url: String,
        pub(crate) session_id: Option<&'a str>,
        pub(crate) _impl: &'a I
    }

    impl<'a, T, I> OdooRequest<'a, T, I>
    where
        T: JsonRpcParams + Debug + Serialize,
        T::Container<T>: Debug + Serialize,
        I: RequestImpl,
    {
        pub(crate) fn new(data: JsonRpcRequest<T>, url: String, session_id: Option<&'a str>, _impl: &'a I) -> Self {
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
                JsonRpcResponse::Success(data) => {
                    Ok(data.result)
                },
                JsonRpcResponse::Error(data) => {
                    Err(data.error.into())
                }
            }
        }
    }
}

mod closure_blocking {
    use std::fmt::{Debug};
    use serde::{Serialize};
    use serde_json::{Value,  to_value};
    use crate::{Result};
    use crate::jsonrpc::{JsonRpcParams};
    use super::{OdooClient, OdooRequest, AuthState, Authed, NotAuthed, RequestImpl};

    /// Convenience typedef. Use this as the return value for your blocking closure
    pub type ClosureResult = Result<(String, Option<String>)>;
    type Closure = Box<dyn Fn(&str, Value, Option<&str>) -> ClosureResult>;

    pub struct ClosureBlocking
    {
        closure: Closure
    }
    impl RequestImpl for ClosureBlocking {}

    impl OdooClient<NotAuthed, ClosureBlocking> {
        pub fn new_closure_blocking<F: Fn(&str, Value, Option<&str>) -> Result<(String, Option<String>)> + 'static>(url: &str, closure: F) -> Self {
            Self::new(
                url,
                ClosureBlocking {
                    closure: Box::new(closure)
                }
            )
        }
    }

    impl<S> OdooClient<S, ClosureBlocking>
    where
        S: AuthState
    {

        pub fn authenticate(self, db: &str, login: &str, password: &str) -> Result<OdooClient<Authed, ClosureBlocking>> {
            let request = self.get_auth_request(db, login, password);
            let (response, session_id) = request.send_internal()?;
            self.parse_auth_response(db, login, password, response, session_id)
        }
    }

    impl<'a, T> OdooRequest<'a, T, ClosureBlocking>
    where
        T: JsonRpcParams + Debug + Serialize,
        T::Container<T>: Debug + Serialize,
    {
        pub fn send(self) -> Result<T::Response> {
            Ok(self.send_internal()?.0)
        }

        fn send_internal(self) -> Result<(T::Response, Option<String>)> {
            let data = to_value(&self.data)?;
            let (response, session_id) = self._impl.closure.as_ref()(&self.url, data, self.session_id)?;
            Ok((self.parse_response(&response)?, session_id))
        }
    }
}

mod closure_async {
    use std::fmt::{Debug};
    use std::future::{Future};
    use std::pin::{Pin};
    use serde::{Serialize};
    use serde_json::{Value,  to_value};
    use crate::{Result};
    use crate::jsonrpc::{JsonRpcParams};
    use super::{OdooClient, OdooRequest, AuthState, Authed, NotAuthed, RequestImpl};

    /// Convenience typedef. Use this as the return value for your async closure
    pub type ClosureResult = Pin<Box<dyn Future<Output = Result<(String, Option<String>)>>>>;
    type Closure = Box<dyn Fn(String, Value, Option<String>) -> ClosureResult>;

    pub struct ClosureAsync
    {
        closure: Closure,

    }
    impl RequestImpl for ClosureAsync {}

    impl OdooClient<NotAuthed, ClosureAsync> {
        pub fn new_closure_async(url: &str, closure: impl 'static + Fn(String, Value, Option<String>) -> Pin<Box<dyn Future<Output = Result<(String, Option<String>)>>>>) -> Self {
            Self::new(
                url,
                ClosureAsync {
                    closure: Box::new(closure)
                }
            )
        }
    }

    impl<S> OdooClient<S, ClosureAsync>
    where
        S: AuthState
    {

        pub async fn authenticate(self, db: &str, login: &str, password: &str) -> Result<OdooClient<Authed, ClosureAsync>> {
            let request = self.get_auth_request(db, login, password);
            let (response, session_id) = request.send_internal().await?;
            self.parse_auth_response(db, login, password, response, session_id)
        }
    }

    impl<'a, T> OdooRequest<'a, T, ClosureAsync>
    where
        T: JsonRpcParams + Debug + Serialize,
        T::Container<T>: Debug + Serialize,
    {
        pub async fn send(self) -> Result<T::Response> {
            Ok(self.send_internal().await?.0)
        }

        async fn send_internal(self) -> Result<(T::Response, Option<String>)> {
            let data = to_value(&self.data)?;
            let (response, session_id) = (self._impl.closure)(
                self.url.clone(),
                data,
                self.session_id.map(|s| s.to_string())
            ).await?;
            Ok((self.parse_response(&response)?, session_id))
        }
    }
}

mod reqwest_blocking {
    use std::fmt::{Debug};
    use serde::{Serialize};
    use reqwest::blocking::{Client};
    use crate::{Result};
    use crate::jsonrpc::{JsonRpcParams};
    use super::{OdooClient, OdooRequest, AuthState, Authed, NotAuthed, RequestImpl};

    pub struct ReqwestBlocking {
        client: Client
    }
    impl RequestImpl for ReqwestBlocking {}

    impl OdooClient<NotAuthed, ReqwestBlocking> {
        pub fn new_reqwest_blocking(url: &str) -> Result<Self> {
            let client = Client::builder()
                .cookie_store(true)
                .build()?;

            Ok(Self::new(
                url,
                ReqwestBlocking {
                    client
                }
            ))
        }
    }

    impl<S> OdooClient<S, ReqwestBlocking>
    where
        S: AuthState
    {

        pub fn authenticate(self, db: &str, login: &str, password: &str) -> Result<OdooClient<Authed, ReqwestBlocking>> {
            let request = self.get_auth_request(db, login, password);
            let (response, session_id) = request.send_internal()?;
            self.parse_auth_response(db, login, password, response, session_id)
        }
    }

    impl<'a, T> OdooRequest<'a, T, ReqwestBlocking>
    where
        T: JsonRpcParams + Debug + Serialize,
        T::Container<T>: Debug + Serialize,
    {
        pub fn send(self) -> Result<T::Response> {
            Ok(self.send_internal()?.0)
        }

        fn send_internal(self) -> Result<(T::Response, Option<String>)> {
            let request = self._impl.client
                .post(&self.url)
                .json(&self.data);
            let response = request.send()?;
            Ok((self.parse_response(&response.text()?)?, None))
        }
    }
}

mod reqwest_async {
    use std::fmt::{Debug};
    use serde::{Serialize};
    use reqwest::{Client};
    use crate::{Result};
    use crate::jsonrpc::{JsonRpcParams};
    use super::{OdooClient, OdooRequest, AuthState, Authed, NotAuthed, RequestImpl};

    pub struct ReqwestAsync {
        client: Client
    }
    impl RequestImpl for ReqwestAsync {}

    impl OdooClient<NotAuthed, ReqwestAsync> {
        pub fn new_reqwest_async(url: &str) -> Result<Self> {
            let client = Client::builder()
                .cookie_store(true)
                .build()?;

            Ok(Self::new(
                url,
                ReqwestAsync {
                    client
                }
            ))
        }
    }

    impl<S> OdooClient<S, ReqwestAsync>
    where
        S: AuthState
    {

        pub async fn authenticate(self, db: &str, login: &str, password: &str) -> Result<OdooClient<Authed, ReqwestAsync>> {
            let request = self.get_auth_request(db, login, password);
            let (response, session_id) = request.send_internal().await?;
            self.parse_auth_response(db, login, password, response, session_id)
        }
    }

    impl<'a, T> OdooRequest<'a, T, ReqwestAsync>
    where
        T: JsonRpcParams + Debug + Serialize,
        T::Container<T>: Debug + Serialize,
    {
        pub async fn send(self) -> Result<T::Response> {
            Ok(self.send_internal().await?.0)
        }

        async fn send_internal(self) -> Result<(T::Response, Option<String>)> {
            let request = self._impl.client
                .post(&self.url)
                .json(&self.data);
            let response = request.send().await?;
            Ok((self.parse_response(&response.text().await?)?, None))
        }
    }
}
