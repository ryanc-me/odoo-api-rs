pub(crate) mod closure_async;
pub(crate) mod closure_blocking;

#[cfg(feature = "async")]
pub(crate) mod reqwest_async;

#[cfg(feature = "blocking")]
pub(crate) mod reqwest_blocking;
