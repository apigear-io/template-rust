mod mqtt_common;
#[allow(unused_imports)]
use tb_same1::api::data_structs::*;
use tb_same1::api::same_enum2_interface::SameEnum2InterfaceTrait;
use tb_same1::implementation::same_enum2_interface::SameEnum2Interface;
use tb_same1::mqtt::same_enum2_interface_client::SameEnum2InterfaceMqttClient;
use tb_same1::mqtt::same_enum2_interface_service::SameEnum2InterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_same_enum2_interface_roundtrip() {
    let impl_ = Arc::new(SameEnum2Interface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-tb_same1-same_enum2_interface");
    let service = Arc::new(SameEnum2InterfaceMqttService::new(impl_.clone() as Arc<dyn SameEnum2InterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload, response_topic, correlation_data| service_drive.handle_message(topic, payload, response_topic, correlation_data));

    let (client_client, client_loop) = mqtt_common::connect("cli-tb_same1-same_enum2_interface");
    let client = Arc::new(SameEnum2InterfaceMqttClient::new(client_client, "cli-tb_same1-same_enum2_interface"));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload, _response_topic, correlation_data| client_drive.handle_message(topic, payload, correlation_data));

    mqtt_common::settle().await;
    let _ = service.publish_current_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func1(Default::default()).await.is_ok());
    assert!(client.func2(Default::default(), Default::default()).await.is_ok());

    // Writable properties: a client set propagates over MQTT to the service implementation.
    {
        let test_value: Enum1Enum = Default::default();
        client.set_prop1(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop1() == test_value).await);
    }
    {
        let test_value: Enum2Enum = Default::default();
        client.set_prop2(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop2() == test_value).await);
    }
}
