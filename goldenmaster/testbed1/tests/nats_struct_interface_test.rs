mod nats_common;
#[allow(unused_imports)]
use testbed1::api::data_structs::*;
use testbed1::api::struct_interface::StructInterfaceTrait;
use testbed1::implementation::struct_interface::StructInterface;
use testbed1::nats::struct_interface_client::StructInterfaceNatsClient;
use testbed1::nats::struct_interface_service::StructInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_struct_interface_roundtrip() {
    let impl_ = Arc::new(StructInterface::default());
    let service = Arc::new(StructInterfaceNatsService::new(impl_.clone() as Arc<dyn StructInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(StructInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func_bool(&Default::default()).await.is_ok());
    assert!(client.func_int(&Default::default()).await.is_ok());
    assert!(client.func_float(&Default::default()).await.is_ok());
    assert!(client.func_string(&Default::default()).await.is_ok());

    // Writable properties: a client set propagates over NATS to the service implementation.
    {
        let test_value: StructBool = Default::default();
        client.set_prop_bool(&test_value.clone());
        assert!(nats_common::wait_until(|| impl_.prop_bool() == test_value).await);
    }
    {
        let test_value: StructInt = Default::default();
        client.set_prop_int(&test_value.clone());
        assert!(nats_common::wait_until(|| impl_.prop_int() == test_value).await);
    }
    {
        let test_value: StructFloat = Default::default();
        client.set_prop_float(&test_value.clone());
        assert!(nats_common::wait_until(|| impl_.prop_float() == test_value).await);
    }
    {
        let test_value: StructString = Default::default();
        client.set_prop_string(&test_value.clone());
        assert!(nats_common::wait_until(|| impl_.prop_string() == test_value).await);
    }
}
