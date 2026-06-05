mod nats_common;
use tb_simple::api::no_signals_interface::NoSignalsInterfaceTrait;
use tb_simple::implementation::no_signals_interface::NoSignalsInterface;
use tb_simple::nats::no_signals_interface_client::NoSignalsInterfaceNatsClient;
use tb_simple::nats::no_signals_interface_service::NoSignalsInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_no_signals_interface_roundtrip() {
    let impl_ = Arc::new(NoSignalsInterface::default());
    let service = Arc::new(NoSignalsInterfaceNatsService::new(impl_.clone() as Arc<dyn NoSignalsInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(NoSignalsInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func_void().await.is_ok());
    assert!(client.func_bool(Default::default()).await.is_ok());

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
