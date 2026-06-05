mod mqtt_common;
use tb_simple::api::no_signals_interface::NoSignalsInterfaceTrait;
use tb_simple::implementation::no_signals_interface::NoSignalsInterface;
use tb_simple::mqtt::no_signals_interface_client::NoSignalsInterfaceMqttClient;
use tb_simple::mqtt::no_signals_interface_service::NoSignalsInterfaceMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_no_signals_interface_roundtrip() {
    let impl_ = Arc::new(NoSignalsInterface::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-tb_simple-no_signals_interface");
    let service = Arc::new(NoSignalsInterfaceMqttService::new(impl_.clone() as Arc<dyn NoSignalsInterfaceTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload, response_topic, correlation_data| service_drive.handle_message(topic, payload, response_topic, correlation_data));

    let (client_client, client_loop) = mqtt_common::connect("cli-tb_simple-no_signals_interface");
    let client = Arc::new(NoSignalsInterfaceMqttClient::new(client_client, "cli-tb_simple-no_signals_interface"));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload, _response_topic, correlation_data| client_drive.handle_message(topic, payload, correlation_data));

    mqtt_common::settle().await;
    let _ = service.publish_current_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.func_void().await.is_ok());
    assert!(client.func_bool(Default::default()).await.is_ok());

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
}
