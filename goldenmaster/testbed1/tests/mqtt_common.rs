#![allow(dead_code)]
use rumqttc::{AsyncClient, Event, EventLoop, MqttOptions, Packet};
use std::sync::Arc;
use std::time::Duration;

/// MQTT broker port (`MQTT_PORT` env var, default `1883`).
fn port() -> u16 {
    std::env::var("MQTT_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(1883)
}

/// Connect a new MQTT client + event loop with the given client id.
pub fn connect(id: &str) -> (Arc<AsyncClient>, EventLoop) {
    let mut opts = MqttOptions::new(id, "127.0.0.1", port());
    opts.set_keep_alive(Duration::from_secs(5));
    let (client, eventloop) = AsyncClient::new(opts, 64);
    (Arc::new(client), eventloop)
}

/// Drive an MQTT event loop in the background, dispatching every incoming
/// publish to `handler`. The returned task runs until the test ends.
pub fn drive<F>(
    mut eventloop: EventLoop,
    handler: F,
) -> tokio::task::JoinHandle<()>
where
    F: Fn(&str, &[u8]) + Send + 'static,
{
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => handler(&p.topic, &p.payload),
                Ok(_) => {}
                Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        }
    })
}

/// Give the broker connection + subscriptions time to establish.
pub async fn settle() {
    tokio::time::sleep(Duration::from_millis(400)).await;
}

/// Poll a condition until it holds or a ~2s timeout elapses.
pub async fn wait_until<F: Fn() -> bool>(cond: F) -> bool {
    for _ in 0..100 {
        if cond() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    cond()
}
