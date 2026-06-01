mod mqtt_common;
use tb_simple::api::void_interface::VoidInterfaceTrait;
use tb_simple::implementation::void_interface::VoidInterface;
use tb_simple::mqtt::void_interface_client::VoidInterfaceMqttClient;
use tb_simple::mqtt::void_interface_service::VoidInterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_void_interface_roundtrip() {
    let impl_ = Arc::new(VoidInterface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-tb_simple-void_interface");
    let service = Arc::new(VoidInterfaceMqttService::new(impl_.clone() as Arc<dyn VoidInterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload| service_drive.handle_message(topic, payload));

    let (client_client, client_loop) = mqtt_common::connect("cli-tb_simple-void_interface");
    let client = Arc::new(VoidInterfaceMqttClient::new(client_client));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload| client_drive.handle_message(topic, payload));

    mqtt_common::settle().await;
    let _ = service.publish_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func_void().await.is_ok());
}
