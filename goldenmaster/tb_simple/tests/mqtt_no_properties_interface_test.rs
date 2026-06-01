mod mqtt_common;
use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTrait;
use tb_simple::implementation::no_properties_interface::NoPropertiesInterface;
use tb_simple::mqtt::no_properties_interface_client::NoPropertiesInterfaceMqttClient;
use tb_simple::mqtt::no_properties_interface_service::NoPropertiesInterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_no_properties_interface_roundtrip() {
    let impl_ = Arc::new(NoPropertiesInterface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-tb_simple-no_properties_interface");
    let service = Arc::new(NoPropertiesInterfaceMqttService::new(impl_.clone() as Arc<dyn NoPropertiesInterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload| service_drive.handle_message(topic, payload));

    let (client_client, client_loop) = mqtt_common::connect("cli-tb_simple-no_properties_interface");
    let client = Arc::new(NoPropertiesInterfaceMqttClient::new(client_client));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload| client_drive.handle_message(topic, payload));

    mqtt_common::settle().await;
    let _ = service.publish_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func_void().await.is_ok());
    assert!(client.func_bool(Default::default()).await.is_ok());
}
