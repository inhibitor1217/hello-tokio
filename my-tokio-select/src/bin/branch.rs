use tokio::time;

async fn action(input: Option<i32>) -> Option<String> {
    let i = input?;

    println!("Sleeping for 1 second... (input: {})", i);
    time::sleep(time::Duration::from_millis(1000)).await;

    Some(format!("action({})", i))
}

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(128);

    tokio::spawn(async move {
        let _ = tx.send(1).await;
        time::sleep(time::Duration::from_millis(300)).await;
        let _ = tx.send(2).await;
        time::sleep(time::Duration::from_millis(300)).await;
        let _ = tx.send(3).await;
        time::sleep(time::Duration::from_millis(300)).await;
        let _ = tx.send(4).await;
        time::sleep(time::Duration::from_millis(300)).await;
        let _ = tx.send(5).await;
    });

    let mut done = false;
    let operation = action(None);
    tokio::pin!(operation);

    loop {
        tokio::select! {
            res = &mut operation, if !done => {
                done = true;
                if let Some(res) = res {
                    println!("operation: {}", res);
                    break;
                }
            }
            Some(v) = rx.recv() => {
                println!("recv: {}", v);
                if v % 2 == 0 {
                    operation.set(action(Some(v)));
                    done = false;
                }
            }
            else => {},
        }
    }
}
