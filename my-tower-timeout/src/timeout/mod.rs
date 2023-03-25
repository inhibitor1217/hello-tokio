use std::{task::Poll, time::Duration};

use tower::Service;

use self::future::ResponseFuture;

pub mod error;
pub mod future;

/// A [`Service`] that adds a timeout to requests.
#[derive(Debug, Clone)]
pub struct Timeout<T> {
    inner: T,
    timeout: Duration,
}

impl<T> Timeout<T> {
    /// Creates a new [`Timeout`] service.
    pub fn new(inner: T, timeout: Duration) -> Self {
        Self { inner, timeout }
    }

    /// Returns a reference to the inner service.
    pub fn get_ref(&self) -> &T {
        &self.inner
    }

    /// Returns a mutable reference to the inner service.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Consumes `self`, returning the inner service.
    pub fn into_inner(self) -> T {
        self.inner
    }
}

impl<T, Request> Service<Request> for Timeout<T>
where
    T: Service<Request>,
    T::Error: Into<crate::BoxError>,
{
    type Response = T::Response;

    type Error = crate::BoxError;

    type Future = ResponseFuture<T::Future>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let res = self.inner.call(req);
        let sleep = tokio::time::sleep(self.timeout);

        ResponseFuture::new(res, sleep)
    }
}
