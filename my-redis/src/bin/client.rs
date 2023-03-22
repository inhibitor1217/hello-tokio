use bytes::Bytes;
use mini_redis::client;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        value: Bytes,
        resp: Responder<()>,
    },
}

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    let connection_manager = tokio::spawn(async move {
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                Command::Get { key, resp } => {
                    let _ = resp.send(client.get(&key).await);
                }
                Command::Set { key, value, resp } => {
                    let _ = resp.send(client.set(&key, value).await);
                }
            }
        }
    });

    let tx1 = tx.clone();
    let task1 = tokio::spawn(async move {
        let (tx, rx) = oneshot::channel();

        tx1.send(Command::Get {
            key: "foo".to_string(),
            resp: tx,
        })
        .await
        .unwrap();

        let result = rx.await;
        println!("GOT = {:?}", result);
    });

    let tx2 = tx.clone();
    let task2 = tokio::spawn(async move {
        let (tx, rx) = oneshot::channel();

        tx2.send(Command::Set {
            key: "foo".to_string(),
            value: "bar".into(),
            resp: tx,
        })
        .await
        .unwrap();

        let result = rx.await;
        println!("GOT = {:?}", result);
    });

    task1.await.unwrap();
    task2.await.unwrap();
    connection_manager.await.unwrap();
}
