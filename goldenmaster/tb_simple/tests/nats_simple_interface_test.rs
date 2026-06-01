mod nats_common;
use tb_simple::api::simple_interface::SimpleInterfaceTrait;
use tb_simple::implementation::simple_interface::SimpleInterface;
use tb_simple::nats::simple_interface_client::SimpleInterfaceNatsClient;
use tb_simple::nats::simple_interface_service::SimpleInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_simple_interface_roundtrip() {
    let impl_ = Arc::new(SimpleInterface::default());
    let service = Arc::new(SimpleInterfaceNatsService::new(impl_.clone() as Arc<dyn SimpleInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();
    let _ = service.publish_state().await;

    let client = Arc::new(SimpleInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
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

    // Writable properties: a client set propagates over NATS to the service implementation.
    {
        let test_value: bool = true;
        client.set_prop_bool(test_value);
        assert!(nats_common::wait_until(|| impl_.prop_bool() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop_int(test_value);
        assert!(nats_common::wait_until(|| impl_.prop_int() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop_int32(test_value);
        assert!(nats_common::wait_until(|| impl_.prop_int32() == test_value).await);
    }
    {
        let test_value: i64 = 1i64;
        client.set_prop_int64(test_value);
        assert!(nats_common::wait_until(|| impl_.prop_int64() == test_value).await);
    }
    {
        let test_value: f32 = 1.0f32;
        client.set_prop_float(test_value);
        assert!(nats_common::wait_until(|| impl_.prop_float() == test_value).await);
    }
    {
        let test_value: f32 = 1.0f32;
        client.set_prop_float32(test_value);
        assert!(nats_common::wait_until(|| impl_.prop_float32() == test_value).await);
    }
    {
        let test_value: f64 = 1.0f64;
        client.set_prop_float64(test_value);
        assert!(nats_common::wait_until(|| impl_.prop_float64() == test_value).await);
    }
    {
        let test_value: String = String::from("test");
        client.set_prop_string(test_value.as_str());
        assert!(nats_common::wait_until(|| impl_.prop_string() == test_value).await);
    }
}
