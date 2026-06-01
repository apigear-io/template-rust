mod mqtt_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
use testbed2::implementation::many_param_interface::ManyParamInterface;
use testbed2::mqtt::many_param_interface_client::ManyParamInterfaceMqttClient;
use testbed2::mqtt::many_param_interface_service::ManyParamInterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_many_param_interface_roundtrip() {
    let impl_ = Arc::new(ManyParamInterface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-testbed2-many_param_interface");
    let service = Arc::new(ManyParamInterfaceMqttService::new(impl_.clone() as Arc<dyn ManyParamInterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload| service_drive.handle_message(topic, payload));

    let (client_client, client_loop) = mqtt_common::connect("cli-testbed2-many_param_interface");
    let client = Arc::new(ManyParamInterfaceMqttClient::new(client_client));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload| client_drive.handle_message(topic, payload));

    mqtt_common::settle().await;
    let _ = service.publish_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func1(Default::default()).await.is_ok());
    assert!(client.func2(Default::default(), Default::default()).await.is_ok());
    assert!(client.func3(Default::default(), Default::default(), Default::default()).await.is_ok());
    assert!(client.func4(Default::default(), Default::default(), Default::default(), Default::default()).await.is_ok());

    // Writable properties: a client set propagates over MQTT to the service implementation.
    {
        let test_value: i32 = 1i32;
        client.set_prop1(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop1() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop2(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop2() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop3(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop3() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop4(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop4() == test_value).await);
    }
}
