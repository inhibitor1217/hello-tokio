use tokio::time;

/// A future that completes after a specified delay.
async fn delay<T>(delay_ms: u64, val: T) -> T {
    time::sleep(time::Duration::from_millis(delay_ms)).await;
    val
}

#[tokio::main]
async fn main() {
    let (mut _tx, mut rx) = tokio::sync::mpsc::channel::<i32>(128);

    let operation = delay(1000, 42);
    tokio::pin!(operation);

    loop {
        tokio::select! {
            _ = &mut operation => break,
            Some(v) = rx.recv() => {
                if v % 2 == 0 {
                    break;
                }
            },
        }
    }
}
