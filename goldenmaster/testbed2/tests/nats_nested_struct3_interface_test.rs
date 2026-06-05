mod nats_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct3_interface::NestedStruct3InterfaceTrait;
use testbed2::implementation::nested_struct3_interface::NestedStruct3Interface;
use testbed2::nats::nested_struct3_interface_client::NestedStruct3InterfaceNatsClient;
use testbed2::nats::nested_struct3_interface_service::NestedStruct3InterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_nested_struct3_interface_roundtrip() {
    let impl_ = Arc::new(NestedStruct3Interface::default());
    let service = Arc::new(NestedStruct3InterfaceNatsService::new(impl_.clone() as Arc<dyn NestedStruct3InterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(NestedStruct3InterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func1(&Default::default()).await.is_ok());
    assert!(client.func2(&Default::default(), &Default::default()).await.is_ok());
    assert!(client.func3(&Default::default(), &Default::default(), &Default::default()).await.is_ok());

    // Writable properties: a client set propagates over NATS to the service implementation.
    {
        let test_value: NestedStruct1 = Default::default();
        client.set_prop1(&test_value.clone());
        assert!(nats_common::wait_until(|| impl_.prop1() == test_value).await);
    }
    {
        let test_value: NestedStruct2 = Default::default();
        client.set_prop2(&test_value.clone());
        assert!(nats_common::wait_until(|| impl_.prop2() == test_value).await);
    }
    {
        let test_value: NestedStruct3 = Default::default();
        client.set_prop3(&test_value.clone());
        assert!(nats_common::wait_until(|| impl_.prop3() == test_value).await);
    }
}
