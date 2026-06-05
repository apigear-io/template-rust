mod mqtt_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
use testbed2::implementation::nested_struct3_interface::NestedStruct3Interface;
use testbed2::mqtt::nested_struct3_interface_client::NestedStruct3InterfaceMqttClient;
use testbed2::mqtt::nested_struct3_interface_service::NestedStruct3InterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_nested_struct3_interface_roundtrip() {
    let impl_ = Arc::new(NestedStruct3Interface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-testbed2-nested_struct3_interface");
    let service = Arc::new(NestedStruct3InterfaceMqttService::new(impl_.clone() as Arc<dyn NestedStruct3InterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload, response_topic, correlation_data| service_drive.handle_message(topic, payload, response_topic, correlation_data));

    let (client_client, client_loop) = mqtt_common::connect("cli-testbed2-nested_struct3_interface");
    let client = Arc::new(NestedStruct3InterfaceMqttClient::new(client_client, "cli-testbed2-nested_struct3_interface"));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload, _response_topic, correlation_data| client_drive.handle_message(topic, payload, correlation_data));

    mqtt_common::settle().await;
    let _ = service.publish_current_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func1(&Default::default()).await.is_ok());
    assert!(client.func2(&Default::default(), &Default::default()).await.is_ok());
    assert!(client.func3(&Default::default(), &Default::default(), &Default::default()).await.is_ok());

    // Writable properties: a client set propagates over MQTT to the service implementation.
    {
        let test_value: NestedStruct1 = Default::default();
        client.set_prop1(&test_value.clone());
        assert!(mqtt_common::wait_until(|| impl_.prop1() == test_value).await);
    }
    {
        let test_value: NestedStruct2 = Default::default();
        client.set_prop2(&test_value.clone());
        assert!(mqtt_common::wait_until(|| impl_.prop2() == test_value).await);
    }
    {
        let test_value: NestedStruct3 = Default::default();
        client.set_prop3(&test_value.clone());
        assert!(mqtt_common::wait_until(|| impl_.prop3() == test_value).await);
    }
}
