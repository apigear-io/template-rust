//! MQTT service example: exposes the generated ManyParamInterface implementation over MQTT.
//!
//! Start a broker (e.g. `mosquitto`), then run this server and, in another terminal,
//! the matching client:
//!     cargo run --bin mqtt_server
//!     cargo run --bin mqtt_client
//! Override the broker port with the MQTT_PORT environment variable (default 1883).
use rumqttc::{AsyncClient, Event, MqttOptions, Packet};
use std::sync::Arc;
use std::time::Duration;
use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
use testbed2::implementation::many_param_interface::ManyParamInterface;
use testbed2::mqtt::many_param_interface_service::ManyParamInterfaceMqttService;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let port: u16 = std::env::var("MQTT_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(1883);
    let mut opts = MqttOptions::new("testbed2-many_param_interface-service", "127.0.0.1", port);
    opts.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(opts, 64);

    let object = Arc::new(ManyParamInterface::default());
    let service = Arc::new(ManyParamInterfaceMqttService::new(object.clone() as Arc<dyn ManyParamInterfaceTrait>, Arc::new(client)));
    service.subscribe_topics().await.expect("subscribe to topics");
    println!("[many_param_interface-mqtt-service] serving on 127.0.0.1:{port} (Ctrl-C to stop)");

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                let _ = service.publish_state().await;
            }
            Ok(Event::Incoming(Packet::Publish(p))) => service.handle_message(&p.topic, &p.payload),
            Ok(_) => {}
            Err(e) => {
                eprintln!("[many_param_interface-mqtt-service] connection error: {e}");
                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        }
    }
}
