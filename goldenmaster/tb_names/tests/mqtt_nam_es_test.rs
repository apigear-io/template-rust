mod mqtt_common;
#[allow(unused_imports)]
use tb_names::api::data_structs::*;
use tb_names::api::nam_es::NamEsTrait;
use tb_names::implementation::nam_es::NamEs;
use tb_names::mqtt::nam_es_client::NamEsMqttClient;
use tb_names::mqtt::nam_es_service::NamEsMqttService;
use std::sync::Arc;

/// End-to-end round-trip over a real MQTT broker: a generated service wraps the
/// implementation, a generated client talks to it, both event loops driven in the
/// background. Ignored by default; the CI integration job starts Mosquitto and runs
/// it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running MQTT broker (MQTT_PORT or 127.0.0.1:1883)"]
async fn test_mqtt_nam_es_roundtrip() {
    let impl_ = Arc::new(NamEs::default());
    let (service_client, service_loop) = mqtt_common::connect("svc-tb_names-nam_es");
    let service = Arc::new(NamEsMqttService::new(impl_.clone() as Arc<dyn NamEsTrait>, service_client));
    service.subscribe_topics().await.expect("service subscribe");
    let service_drive = service.clone();
    let _service_handle = mqtt_common::drive(service_loop, move |topic, payload| service_drive.handle_message(topic, payload));

    let (client_client, client_loop) = mqtt_common::connect("cli-tb_names-nam_es");
    let client = Arc::new(NamEsMqttClient::new(client_client));
    client.subscribe_topics().await.expect("client subscribe");
    let client_drive = client.clone();
    let _client_handle = mqtt_common::drive(client_loop, move |topic, payload| client_drive.handle_message(topic, payload));

    mqtt_common::settle().await;
    let _ = service.publish_state().await;

    // Operations: published as MQTT requests and delivered to the broker.
    assert!(client.some_function(Default::default()).await.is_ok());
    assert!(client.some_function2(Default::default()).await.is_ok());

    // Writable properties: a client set propagates over MQTT to the service implementation.
    {
        let test_value: bool = true;
        client.set_switch(test_value);
        assert!(mqtt_common::wait_until(|| impl_.switch() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_some_property(test_value);
        assert!(mqtt_common::wait_until(|| impl_.some_property() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_some_poperty2(test_value);
        assert!(mqtt_common::wait_until(|| impl_.some_poperty2() == test_value).await);
    }
    {
        let test_value: Enum_With_Under_scoresEnum = Default::default();
        client.set_enum_property(test_value);
        assert!(mqtt_common::wait_until(|| impl_.enum_property() == test_value).await);
    }
}
