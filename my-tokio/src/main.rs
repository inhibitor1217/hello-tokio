use std::time::{Duration, Instant};

use delay::Delay;

mod delay;
mod mini_tokio;

#[tokio::main]
async fn main() {
    let when = Instant::now() + Duration::from_millis(10);
    let future = Delay::new(when);

    println!("Current time: {:?}", Instant::now());
    future.await;
    println!("Current time: {:?}", Instant::now());
}
