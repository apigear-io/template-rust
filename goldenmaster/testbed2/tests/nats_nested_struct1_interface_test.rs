mod nats_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::nested_struct1_interface::NestedStruct1InterfaceTrait;
use testbed2::implementation::nested_struct1_interface::NestedStruct1Interface;
use testbed2::nats::nested_struct1_interface_client::NestedStruct1InterfaceNatsClient;
use testbed2::nats::nested_struct1_interface_service::NestedStruct1InterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_nested_struct1_interface_roundtrip() {
    let impl_ = Arc::new(NestedStruct1Interface::default());
    let service = Arc::new(NestedStruct1InterfaceNatsService::new(impl_.clone() as Arc<dyn NestedStruct1InterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(NestedStruct1InterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func_no_return_value(&Default::default()).await.is_ok());
    assert!(client.func_no_params().await.is_ok());
    assert!(client.func1(&Default::default()).await.is_ok());

    // Writable properties: a client set propagates over NATS to the service implementation.
    {
        let test_value: NestedStruct1 = Default::default();
        client.set_prop1(&test_value.clone());
        assert!(nats_common::wait_until(|| impl_.prop1() == test_value).await);
    }
}
