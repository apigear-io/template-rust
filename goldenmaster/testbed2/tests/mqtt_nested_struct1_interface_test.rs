mod mqtt_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
use testbed2::implementation::nested_struct1_interface::NestedStruct1Interface;
use testbed2::mqtt::nested_struct1_interface_client::NestedStruct1InterfaceMqttClient;
use testbed2::mqtt::nested_struct1_interface_service::NestedStruct1InterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_nested_struct1_interface_roundtrip() {
    let impl_ = Arc::new(NestedStruct1Interface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-testbed2-nested_struct1_interface");
    let service = Arc::new(NestedStruct1InterfaceMqttService::new(impl_.clone() as Arc<dyn NestedStruct1InterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload, response_topic, correlation_data| service_drive.handle_message(topic, payload, response_topic, correlation_data));

    let (client_client, client_loop) = mqtt_common::connect("cli-testbed2-nested_struct1_interface");
    let client = Arc::new(NestedStruct1InterfaceMqttClient::new(client_client, "cli-testbed2-nested_struct1_interface"));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload, _response_topic, correlation_data| client_drive.handle_message(topic, payload, correlation_data));

    mqtt_common::settle().await;
    let _ = service.publish_current_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func_no_return_value(&Default::default()).await.is_ok());
    assert!(client.func_no_params().await.is_ok());
    assert!(client.func1(&Default::default()).await.is_ok());

    // Writable properties: a client set propagates over MQTT to the service implementation.
    {
        let test_value: NestedStruct1 = Default::default();
        client.set_prop1(&test_value.clone());
        assert!(mqtt_common::wait_until(|| impl_.prop1() == test_value).await);
    }
}
