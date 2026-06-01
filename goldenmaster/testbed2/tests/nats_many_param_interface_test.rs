mod nats_common;
#[allow(unused_imports)]
use testbed2::api::data_structs::*;
use testbed2::api::many_param_interface::ManyParamInterfaceTrait;
use testbed2::implementation::many_param_interface::ManyParamInterface;
use testbed2::nats::many_param_interface_client::ManyParamInterfaceNatsClient;
use testbed2::nats::many_param_interface_service::ManyParamInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_many_param_interface_roundtrip() {
    let impl_ = Arc::new(ManyParamInterface::default());
    let service = Arc::new(ManyParamInterfaceNatsService::new(impl_.clone() as Arc<dyn ManyParamInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();
    let _ = service.publish_state().await;

    let client = Arc::new(ManyParamInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func1(Default::default()).await.is_ok());
    assert!(client.func2(Default::default(), Default::default()).await.is_ok());
    assert!(client.func3(Default::default(), Default::default(), Default::default()).await.is_ok());
    assert!(client.func4(Default::default(), Default::default(), Default::default(), Default::default()).await.is_ok());

    // Writable properties: a client set propagates over NATS to the service implementation.
    {
        let test_value: i32 = 1i32;
        client.set_prop1(test_value);
        assert!(nats_common::wait_until(|| impl_.prop1() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop2(test_value);
        assert!(nats_common::wait_until(|| impl_.prop2() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop3(test_value);
        assert!(nats_common::wait_until(|| impl_.prop3() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_prop4(test_value);
        assert!(nats_common::wait_until(|| impl_.prop4() == test_value).await);
    }
}
