mod mqtt_common;
#[allow(unused_imports)]
use testbed1::api::data_structs::*;
use testbed1::api::struct_array_interface::StructArrayInterfaceTrait;
use testbed1::implementation::struct_array_interface::StructArrayInterface;
use testbed1::mqtt::struct_array_interface_client::StructArrayInterfaceMqttClient;
use testbed1::mqtt::struct_array_interface_service::StructArrayInterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_struct_array_interface_roundtrip() {
    let impl_ = Arc::new(StructArrayInterface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-testbed1-struct_array_interface");
    let service = Arc::new(StructArrayInterfaceMqttService::new(impl_.clone() as Arc<dyn StructArrayInterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload, response_topic, correlation_data| service_drive.handle_message(topic, payload, response_topic, correlation_data));

    let (client_client, client_loop) = mqtt_common::connect("cli-testbed1-struct_array_interface");
    let client = Arc::new(StructArrayInterfaceMqttClient::new(client_client, "cli-testbed1-struct_array_interface"));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload, _response_topic, correlation_data| client_drive.handle_message(topic, payload, correlation_data));

    mqtt_common::settle().await;
    let _ = service.publish_current_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func_bool(Default::default()).await.is_ok());
    assert!(client.func_int(Default::default()).await.is_ok());
    assert!(client.func_float(Default::default()).await.is_ok());
    assert!(client.func_string(Default::default()).await.is_ok());
    assert!(client.func_enum(Default::default()).await.is_ok());

    // Writable properties: a client set propagates over MQTT to the service implementation.
    {
        let test_value: Vec<StructBool> = vec![Default::default()];
        client.set_prop_bool(test_value.as_slice());
        assert!(mqtt_common::wait_until(|| impl_.prop_bool() == test_value).await);
    }
    {
        let test_value: Vec<StructInt> = vec![Default::default()];
        client.set_prop_int(test_value.as_slice());
        assert!(mqtt_common::wait_until(|| impl_.prop_int() == test_value).await);
    }
    {
        let test_value: Vec<StructFloat> = vec![Default::default()];
        client.set_prop_float(test_value.as_slice());
        assert!(mqtt_common::wait_until(|| impl_.prop_float() == test_value).await);
    }
    {
        let test_value: Vec<StructString> = vec![Default::default()];
        client.set_prop_string(test_value.as_slice());
        assert!(mqtt_common::wait_until(|| impl_.prop_string() == test_value).await);
    }
    {
        let test_value: Vec<Enum0Enum> = vec![Default::default()];
        client.set_prop_enum(test_value.as_slice());
        assert!(mqtt_common::wait_until(|| impl_.prop_enum() == test_value).await);
    }
}
