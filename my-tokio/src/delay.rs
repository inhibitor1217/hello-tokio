use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
    thread,
    time::Instant,
};

pub struct Delay {
    when: Instant,
}

impl Delay {
    pub fn new(when: Instant) -> Delay {
        Delay { when }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            let waker = cx.waker().clone();
            let when = self.when;

            thread::spawn(move || {
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }
                waker.wake();
            });

            Poll::Pending
        }
    }
}
