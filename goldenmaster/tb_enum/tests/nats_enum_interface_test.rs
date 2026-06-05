mod nats_common;
#[allow(unused_imports)]
use tb_enum::api::data_structs::*;
use tb_enum::api::enum_interface::EnumInterfaceTrait;
use tb_enum::implementation::enum_interface::EnumInterface;
use tb_enum::nats::enum_interface_client::EnumInterfaceNatsClient;
use tb_enum::nats::enum_interface_service::EnumInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_enum_interface_roundtrip() {
    let impl_ = Arc::new(EnumInterface::default());
    let service = Arc::new(EnumInterfaceNatsService::new(impl_.clone() as Arc<dyn EnumInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(EnumInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func0(Default::default()).await.is_ok());
    assert!(client.func1(Default::default()).await.is_ok());
    assert!(client.func2(Default::default()).await.is_ok());
    assert!(client.func3(Default::default()).await.is_ok());

    // Writable properties: a client set propagates over NATS to the service implementation.
    {
        let test_value: Enum0Enum = Default::default();
        client.set_prop0(test_value);
        assert!(nats_common::wait_until(|| impl_.prop0() == test_value).await);
    }
    {
        let test_value: Enum1Enum = Default::default();
        client.set_prop1(test_value);
        assert!(nats_common::wait_until(|| impl_.prop1() == test_value).await);
    }
    {
        let test_value: Enum2Enum = Default::default();
        client.set_prop2(test_value);
        assert!(nats_common::wait_until(|| impl_.prop2() == test_value).await);
    }
    {
        let test_value: Enum3Enum = Default::default();
        client.set_prop3(test_value);
        assert!(nats_common::wait_until(|| impl_.prop3() == test_value).await);
    }
}
