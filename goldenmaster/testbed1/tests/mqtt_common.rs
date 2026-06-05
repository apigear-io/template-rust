#![allow(dead_code)]
use rumqttc::v5::mqttbytes::v5::Packet;
use rumqttc::v5::{AsyncClient, Event, EventLoop, MqttOptions};
use std::sync::Arc;
use std::time::Duration;

/// MQTT broker port (`MQTT_PORT` env var, default `1883`).
fn port() -> u16 {
    std::env::var("MQTT_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(1883)
}

/// Connect a new MQTT v5 client + event loop with the given client id.
pub fn connect(id: &str) -> (Arc<AsyncClient>, EventLoop) {
    let mut opts = MqttOptions::new(id, "127.0.0.1", port());
    opts.set_keep_alive(Duration::from_secs(5));
    let (client, eventloop) = AsyncClient::new(opts, 64);
    (Arc::new(client), eventloop)
}

/// Drive an MQTT event loop in the background, dispatching every incoming publish
/// (topic, payload, MQTT 5 response-topic, MQTT 5 correlation-data) to `handler`.
/// The returned task runs until the test ends.
pub fn drive<F>(
    mut eventloop: EventLoop,
    handler: F,
) -> tokio::task::JoinHandle<()>
where
    F: Fn(&str, &[u8], Option<&str>, Option<&[u8]>) + Send + 'static,
{
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let topic = String::from_utf8_lossy(&p.topic);
                    let (rt, cd) = match &p.properties {
                        Some(pr) => (pr.response_topic.as_deref(), pr.correlation_data.as_deref()),
                        None => (None, None),
                    };
                    handler(&topic, &p.payload, rt, cd);
                }
                Ok(_) => {}
                Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        }
    })
}

/// Give the broker connection + subscriptions time to establish (generous for CI).
pub async fn settle() {
    tokio::time::sleep(Duration::from_millis(1000)).await;
}

/// Poll a condition until it holds or a ~6s timeout elapses (generous for CI).
pub async fn wait_until<F: Fn() -> bool>(cond: F) -> bool {
    for _ in 0..300 {
        if cond() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
    cond()
}
