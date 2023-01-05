//! Internal module to make the `client.rs` file more readable

use super::OdooRequest;
use crate::jsonrpc::{JsonRpcParams, OdooId, OdooWebMethod};
use crate::service::web::{SessionAuthenticate, SessionAuthenticateResponse};
use crate::Result;
use serde::Serialize;
use serde_json::{from_str, to_string};
use std::fmt::Debug;

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
/// use odoo_api::{OdooClient, jvec, jmap};
///
/// # async fn test() -> odoo_api::Result<()> {
/// let url = "https://demo.odoo.com";
/// let client = OdooClient::new_reqwest_async(url)?
///     .authenticate(
///         "test-database",
///         "admin",
///         "password"
///     ).await?;
///
/// let user_ids = client.execute(
///     "res.users",
///     "search",
///     jvec![
///         []
///     ]
/// ).send().await?;
///
/// println!("Found user IDs: {:?}", user_ids.data);
/// # Ok(())
/// # }
/// ```
pub struct OdooClient<S, I>
where
    S: AuthState,
    I: RequestImpl,
{
    pub(crate) url: String,
    pub(crate) url_jsonrpc: String,

    pub(crate) auth: S,
    pub(crate) _impl: I,
}

// Base client methods
impl<S, I> OdooClient<S, I>
where
    S: AuthState,
    I: RequestImpl,
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
        S: AuthState,
    {
        OdooRequest::new(data.build(), url.into(), self.session_id(), &self._impl)
    }

    /// Helper method to perform the 1st stage of the authentication request
    ///
    /// Implementors of [`RequestImpl`] will use this method to build an
    /// [`OdooRequest`], which they will then send using their own `send()` method.
    ///
    /// This is necessary because each `RequestImpl` has its own `send()` signature
    /// (i.e., some are `fn send()`, some are `async fn send()`).
    pub(crate) fn get_auth_request(
        &self,
        db: &str,
        login: &str,
        password: &str,
    ) -> OdooRequest<SessionAuthenticate, I> {
        let authenticate = crate::service::web::SessionAuthenticate {
            db: db.into(),
            login: login.into(),
            password: password.into(),
        };
        let url_frag = authenticate.describe();

        self.build_request(authenticate, &format!("{}{}", &self.url, url_frag))
    }

    /// Helper method to perform the 2nd stage of the authentication request
    ///
    /// At this point, the [`OdooRequest`] has been sent by the [`RequestImpl`],
    /// and the response data has been fetched and parsed.
    ///
    /// This method extracts the `uid` and `session_id` from the resulting request,
    /// and returns an `OdooClient<Authed, I>`, e.g., an "authenticated" client.
    pub(crate) fn parse_auth_response(
        self,
        db: &str,
        login: &str,
        password: &str,
        response: SessionAuthenticateResponse,
        session_id: Option<String>,
    ) -> Result<OdooClient<Authed, I>> {
        let uid = response
            .data
            .get("uid")
            .ok_or("Failed to parse UID from /web/session/authenticate call")?;
        //TODO: this is a bit awkward..
        let uid = from_str(&to_string(uid)?)?;
        let auth = Authed {
            database: db.into(),
            uid,
            login: login.into(),
            password: password.into(),
            session_id,
        };

        Ok(OdooClient {
            url: self.url,
            url_jsonrpc: self.url_jsonrpc,
            auth,
            _impl: self._impl,
        })
    }

    pub fn session_id(&self) -> Option<&str> {
        self.auth.get_session_id()
    }

    pub fn authenticate_manual(
        self,
        db: &str,
        login: &str,
        uid: OdooId,
        password: &str,
        session_id: Option<String>,
    ) -> OdooClient<Authed, I> {
        let auth = Authed {
            database: db.into(),
            uid,
            login: login.into(),
            password: password.into(),
            session_id,
        };

        OdooClient {
            url: self.url,
            url_jsonrpc: self.url_jsonrpc,
            auth,
            _impl: self._impl,
        }
    }
}

/// Methods for non-authenticated clients
impl<I> OdooClient<NotAuthed, I>
where
    I: RequestImpl,
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
            _impl,
        }
    }
}
