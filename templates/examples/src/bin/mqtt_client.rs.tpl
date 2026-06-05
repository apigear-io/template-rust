{{- $module := index .System.Modules 0 }}
{{- $interface := index $module.Interfaces 0 -}}
//! MQTT client example: talks to the {{Camel $interface.Name}} MQTT service.
//!
//! Start a broker and `mqtt_server` first, then:
//!     cargo run --bin mqtt_client
//! Override the broker port with the MQTT_PORT environment variable (default 1883).
#![allow(unused_imports, unused_variables)]
use rumqttc::v5::mqttbytes::v5::Packet;
use rumqttc::v5::{AsyncClient, Event, MqttOptions};
use std::sync::Arc;
use std::time::Duration;
use {{snake $module.Name}}::api::{{snake $interface.Name}}::{{Camel $interface.Name}}Trait;
use {{snake $module.Name}}::mqtt::{{snake $interface.Name}}_client::{{Camel $interface.Name}}MqttClient;

#[tokio::main(flavor = "multi_thread")]
async fn main() {
    let port: u16 = std::env::var("MQTT_PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(1883);
    let client_id = "{{snake $module.Name}}-{{snake $interface.Name}}-client";
    let mut opts = MqttOptions::new(client_id, "127.0.0.1", port);
    opts.set_keep_alive(Duration::from_secs(5));
    let (mqtt, mut eventloop) = AsyncClient::new(opts, 64);

    let client = Arc::new({{Camel $interface.Name}}MqttClient::new(Arc::new(mqtt), client_id));
    client.subscribe_topics().await.expect("subscribe to topics");

    // Drive the MQTT event loop so the client receives RPC replies, property changes and signals.
    let pump = client.clone();
    tokio::spawn(async move {
        loop {
            match eventloop.poll().await {
                Ok(Event::Incoming(Packet::Publish(p))) => {
                    let topic = String::from_utf8_lossy(&p.topic);
                    let correlation_data = p.properties.as_ref().and_then(|pr| pr.correlation_data.as_deref());
                    pump.handle_message(&topic, &p.payload, correlation_data);
                }
                Ok(_) => {}
                Err(_) => tokio::time::sleep(Duration::from_millis(200)).await,
            }
        }
    });

    // Give the connection time to establish and the retained state to arrive.
    tokio::time::sleep(Duration::from_millis(800)).await;
    println!("[{{snake $interface.Name}}-mqtt-client] connected to 127.0.0.1:{port}");
{{- range $i, $e := $interface.Operations }}
{{- if not $i }}

    // Invoke the first operation (published as an MQTT request; reply awaited).
    let result = client.{{snake .Name }}(
{{- range $j, $p := .Params }}
{{-   if $j }}, {{ end -}}
{{-   $isComplex := or ( and (eq false .IsPrimitive) (eq false .IsEnum) ) (eq true .IsArray) (eq "string" .Type) -}}
{{    if and (eq false .IsArray) (ne "string" .Type) $isComplex }}&{{end -}}
Default::default()
{{- end -}}
).await;
    println!("[{{snake $interface.Name}}-mqtt-client] {{snake .Name}}() -> {:?}", result);
{{- end }}
{{- end }}
{{- if len $interface.Properties }}

    // Read the property values received from the service.
{{- range $interface.Properties }}
    println!("  {{snake .Name}} = {:?}", client.{{snake .Name }}());
{{- end }}
{{- end }}
}
