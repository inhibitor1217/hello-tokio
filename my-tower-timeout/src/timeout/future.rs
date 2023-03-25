use std::{future::Future, task::Poll};

use super::error;

#[derive(Debug)]
#[pin_project::pin_project]
pub struct ResponseFuture<F> {
    #[pin]
    future: F,
    #[pin]
    sleep: tokio::time::Sleep,
}

impl<F> ResponseFuture<F> {
    pub(super) fn new(future: F, sleep: tokio::time::Sleep) -> Self {
        Self { future, sleep }
    }
}

impl<F, Response, Error> Future for ResponseFuture<F>
where
    F: Future<Output = Result<Response, Error>>,
    Error: Into<crate::BoxError>,
{
    type Output = Result<Response, crate::BoxError>;

    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.future.poll(cx) {
            Poll::Ready(res) => return Poll::Ready(res.map_err(Into::into)),
            Poll::Pending => {}
        }

        match this.sleep.poll(cx) {
            Poll::Ready(_) => return Poll::Ready(Err(Box::new(error::TimeoutError(())))),
            Poll::Pending => {}
        }

        Poll::Pending
    }
}
