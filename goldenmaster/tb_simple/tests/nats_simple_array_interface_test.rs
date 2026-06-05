mod nats_common;
use tb_simple::api::simple_array_interface::SimpleArrayInterfaceTrait;
use tb_simple::implementation::simple_array_interface::SimpleArrayInterface;
use tb_simple::nats::simple_array_interface_client::SimpleArrayInterfaceNatsClient;
use tb_simple::nats::simple_array_interface_service::SimpleArrayInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_simple_array_interface_roundtrip() {
    let impl_ = Arc::new(SimpleArrayInterface::default());
    let service = Arc::new(SimpleArrayInterfaceNatsService::new(impl_.clone() as Arc<dyn SimpleArrayInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(SimpleArrayInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
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
        let test_value: Vec<bool> = vec![Default::default()];
        client.set_prop_bool(test_value.as_slice());
        assert!(nats_common::wait_until(|| impl_.prop_bool() == test_value).await);
    }
    {
        let test_value: Vec<i32> = vec![Default::default()];
        client.set_prop_int(test_value.as_slice());
        assert!(nats_common::wait_until(|| impl_.prop_int() == test_value).await);
    }
    {
        let test_value: Vec<i32> = vec![Default::default()];
        client.set_prop_int32(test_value.as_slice());
        assert!(nats_common::wait_until(|| impl_.prop_int32() == test_value).await);
    }
    {
        let test_value: Vec<i64> = vec![Default::default()];
        client.set_prop_int64(test_value.as_slice());
        assert!(nats_common::wait_until(|| impl_.prop_int64() == test_value).await);
    }
    {
        let test_value: Vec<f32> = vec![Default::default()];
        client.set_prop_float(test_value.as_slice());
        assert!(nats_common::wait_until(|| impl_.prop_float() == test_value).await);
    }
    {
        let test_value: Vec<f32> = vec![Default::default()];
        client.set_prop_float32(test_value.as_slice());
        assert!(nats_common::wait_until(|| impl_.prop_float32() == test_value).await);
    }
    {
        let test_value: Vec<f64> = vec![Default::default()];
        client.set_prop_float64(test_value.as_slice());
        assert!(nats_common::wait_until(|| impl_.prop_float64() == test_value).await);
    }
    {
        let test_value: Vec<String> = vec![Default::default()];
        client.set_prop_string(test_value.as_slice());
        assert!(nats_common::wait_until(|| impl_.prop_string() == test_value).await);
    }
}
