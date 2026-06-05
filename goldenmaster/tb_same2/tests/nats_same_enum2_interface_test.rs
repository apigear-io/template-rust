mod nats_common;
#[allow(unused_imports)]
use tb_same2::api::data_structs::*;
use tb_same2::api::same_enum2_interface::SameEnum2InterfaceTrait;
use tb_same2::implementation::same_enum2_interface::SameEnum2Interface;
use tb_same2::nats::same_enum2_interface_client::SameEnum2InterfaceNatsClient;
use tb_same2::nats::same_enum2_interface_service::SameEnum2InterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_same_enum2_interface_roundtrip() {
    let impl_ = Arc::new(SameEnum2Interface::default());
    let service = Arc::new(SameEnum2InterfaceNatsService::new(impl_.clone() as Arc<dyn SameEnum2InterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(SameEnum2InterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func1(Default::default()).await.is_ok());
    assert!(client.func2(Default::default(), Default::default()).await.is_ok());

    // Writable properties: a client set propagates over NATS to the service implementation.
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
}
