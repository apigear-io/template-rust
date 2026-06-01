mod nats_common;
use tb_simple::api::no_operations_interface::NoOperationsInterfaceTrait;
use tb_simple::implementation::no_operations_interface::NoOperationsInterface;
use tb_simple::nats::no_operations_interface_client::NoOperationsInterfaceNatsClient;
use tb_simple::nats::no_operations_interface_service::NoOperationsInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_no_operations_interface_roundtrip() {
    let impl_ = Arc::new(NoOperationsInterface::default());
    let service = Arc::new(NoOperationsInterfaceNatsService::new(impl_.clone() as Arc<dyn NoOperationsInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();
    let _ = service.publish_state().await;

    let client = Arc::new(NoOperationsInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

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
}
