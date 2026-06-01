mod mqtt_common;
use tb_simple::api::simple_interface::SimpleInterfaceTrait;
use tb_simple::implementation::simple_interface::SimpleInterface;
use tb_simple::mqtt::simple_interface_client::SimpleInterfaceMqttClient;
use tb_simple::mqtt::simple_interface_service::SimpleInterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_simple_interface_roundtrip() {
    let impl_ = Arc::new(SimpleInterface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-tb_simple-simple_interface");
    let service = Arc::new(SimpleInterfaceMqttService::new(impl_.clone() as Arc<dyn SimpleInterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload| service_drive.handle_message(topic, payload));

    let (client_client, client_loop) = mqtt_common::connect("cli-tb_simple-simple_interface");
    let client = Arc::new(SimpleInterfaceMqttClient::new(client_client));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload| client_drive.handle_message(topic, payload));

    mqtt_common::settle().await;
    let _ = service.publish_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func_no_return_value(Default::default()).await.is_ok());
    assert!(client.func_no_params().await.is_ok());
    assert!(client.func_bool(Default::default()).await.is_ok());
    assert!(client.func_int(Default::default()).await.is_ok());
    assert!(client.func_int32(Default::default()).await.is_ok());
    assert!(client.func_int64(Default::default()).await.is_ok());
    assert!(client.func_float(Default::default()).await.is_ok());
    assert!(client.func_float32(Default::default()).await.is_ok());
    assert!(client.func_float64(Default::default()).await.is_ok());
    assert!(client.func_string(Default::default()).await.is_ok());

    // Writable properties: a client set propagates over MQTT to the service implementation.
    {
        let test_value: bool = true;
        client.set_prop_bool(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop_bool() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop_int(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop_int() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop_int32(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop_int32() == test_value).await);
    }
    {
        let test_value: i64 = 1i64;
        client.set_prop_int64(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop_int64() == test_value).await);
    }
    {
        let test_value: f32 = 1.0f32;
        client.set_prop_float(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop_float() == test_value).await);
    }
    {
        let test_value: f32 = 1.0f32;
        client.set_prop_float32(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop_float32() == test_value).await);
    }
    {
        let test_value: f64 = 1.0f64;
        client.set_prop_float64(test_value);
        assert!(mqtt_common::wait_until(|| impl_.prop_float64() == test_value).await);
    }
    {
        let test_value: String = String::from("test");
        client.set_prop_string(test_value.as_str());
        assert!(mqtt_common::wait_until(|| impl_.prop_string() == test_value).await);
    }
}
