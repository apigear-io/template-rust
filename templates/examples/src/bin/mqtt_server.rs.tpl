{{- $module := index .System.Modules 0 }}
{{- $interface := index $module.Interfaces 0 -}}
//! MQTT service example: exposes the generated {{Camel $interface.Name}} implementation over MQTT.
//!
//! Start a broker (e.g. `mosquitto`), then run this server and, in another terminal,
//! the matching client:
//!     cargo run --bin mqtt_server
//!     cargo run --bin mqtt_client
//! Override the broker port with the MQTT_PORT environment variable (default 1883).
use rumqttc::v5::mqttbytes::v5::Packet;
use rumqttc::v5::{AsyncClient, Event, MqttOptions};
use std::sync::Arc;
use std::time::Duration;
use {{snake $module.Name}}::api::{{snake $interface.Name}}::{{Camel $interface.Name}}Trait;
use {{snake $module.Name}}::implementation::{{snake $interface.Name}}::{{Camel $interface.Name}};
use {{snake $module.Name}}::mqtt::{{snake $interface.Name}}_service::{{Camel $interface.Name}}MqttService;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let port: u16 = std::env::var("MQTT_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(1883);
    let mut opts = MqttOptions::new("{{snake $module.Name}}-{{snake $interface.Name}}-service", "127.0.0.1", port);
    opts.set_keep_alive(Duration::from_secs(5));
    let (client, mut eventloop) = AsyncClient::new(opts, 64);

    let object = Arc::new({{Camel $interface.Name}}::default());
    let service = Arc::new({{Camel $interface.Name}}MqttService::new(object.clone() as Arc<dyn {{Camel $interface.Name}}Trait>, Arc::new(client)));
    service.subscribe_topics().await.expect("subscribe to topics");
    println!("[{{snake $interface.Name}}-mqtt-service] serving on 127.0.0.1:{port} (Ctrl-C to stop)");

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Packet::ConnAck(_))) => {
                // Re-publish retained property state for clients that (re)connect.
                let _ = service.publish_current_state().await;
            }
            Ok(Event::Incoming(Packet::Publish(p))) => {
                let topic = String::from_utf8_lossy(&p.topic);
                let (response_topic, correlation_data) = match &p.properties {
                    Some(pr) => (pr.response_topic.as_deref(), pr.correlation_data.as_deref()),
                    None => (None, None),
                };
                service.handle_message(&topic, &p.payload, response_topic, correlation_data);
            }
            Ok(_) => {}
            Err(e) => {
                eprintln!("[{{snake $interface.Name}}-mqtt-service] connection error: {e}");
                tokio::time::sleep(Duration::from_millis(300)).await;
            }
        }
    }
}
