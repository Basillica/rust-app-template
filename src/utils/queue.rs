use futures_util::StreamExt;
use tokio::sync::mpsc;
use tracing::info;


pub async fn handler_sender(client: async_nats::Client, mut recv: mpsc::Receiver<String>) -> Result<(), async_nats::Error> {
    loop {
        while let Some(msg) = recv.recv().await {
            println!("message received: {}", msg);
            match client.publish("easydev2.publish", msg.into()).await {
                Ok(()) => println!("successfully published message"),
                Err(e) => println!("error: {:?}", e)
            }
        }
    }
}

pub async fn send_to_nats(client: async_nats::Client) -> Result<(), async_nats::Error> {
    loop {
        for subject in ["easydev.topic1", "easydev.topic2", "easydev.topic3"] {
            client.publish(subject, "hello from easydev".into()).await?;
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        info!("completed sending cycle")
    }
}

pub async fn receive_from_nats(client: async_nats::Client) -> Result<(), async_nats::Error> {
    let mut subscription = client.subscribe("easydev.*").await?;
    loop {
        while let Some(msg) = subscription.next().await {
            println!("{:?} received message on {:?}", std::str::from_utf8(&msg.payload), &msg.subject)
        }
    }
}


pub async fn handle_nats_messages(client: async_nats::Client) -> Result<(), async_nats::Error> {
    let mut subscription = client.subscribe("easydev2.*").await?;
    loop {
        while let Some(msg) = subscription.next().await {
            println!("{:?} received message on {:?}", std::str::from_utf8(&msg.payload), &msg.subject)
        }
    }
}