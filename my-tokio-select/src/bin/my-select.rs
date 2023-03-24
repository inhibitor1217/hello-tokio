use std::{future::Future, pin::Pin, task::Poll};

use tokio::sync::oneshot;

struct MySelect {
    rx1: oneshot::Receiver<&'static str>,
    rx2: oneshot::Receiver<&'static str>,
}

impl Future for MySelect {
    type Output = ();

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if let Poll::Ready(val) = Pin::new(&mut self.rx1).poll(cx) {
            println!("rx1: {:?}", val);
            Poll::Ready(())
        } else if let Poll::Ready(val) = Pin::new(&mut self.rx2).poll(cx) {
            println!("rx2: {:?}", val);
            Poll::Ready(())
        } else {
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx1, rx1) = oneshot::channel();
    let (tx2, rx2) = oneshot::channel();

    tokio::spawn(async move {
        let _ = tx1.send("Hello");
    });

    tokio::spawn(async move {
        let _ = tx2.send("World");
    });

    MySelect { rx1, rx2 }.await;
}
