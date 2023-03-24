use tokio_stream::StreamExt;

async fn publish() -> mini_redis::Result<()> {
    let mut client = mini_redis::client::connect("127.0.0.1:6379").await?;

    client.publish("numbers", "1".into()).await?;
    client.publish("numbers", "two".into()).await?;
    client.publish("numbers", "3".into()).await?;
    client.publish("numbers", "four".into()).await?;
    client.publish("numbers", "five".into()).await?;
    client.publish("numbers", "6".into()).await?;

    Ok(())
}

async fn subscribe() -> mini_redis::Result<()> {
    let client = mini_redis::client::connect("127.0.0.1:6379").await?;
    let subscriber = client.subscribe(vec!["numbers".into()]).await?;
    let messages = subscriber
        .into_stream()
        .filter(|msg| match msg {
            Ok(msg) => msg.content.len() == 1,
            _ => false,
        })
        .map(|msg| msg.unwrap().content)
        .take(3);
    tokio::pin!(messages);

    while let Some(msg) = messages.next().await {
        println!("Got: {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> mini_redis::Result<()> {
    tokio::spawn(async { publish().await });
    subscribe().await?;

    Ok(())
}
