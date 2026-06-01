mod nats_common;
use tb_simple::api::void_interface::VoidInterfaceTrait;
use tb_simple::implementation::void_interface::VoidInterface;
use tb_simple::nats::void_interface_client::VoidInterfaceNatsClient;
use tb_simple::nats::void_interface_service::VoidInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_void_interface_roundtrip() {
    let impl_ = Arc::new(VoidInterface::default());
    let service = Arc::new(VoidInterfaceNatsService::new(impl_.clone() as Arc<dyn VoidInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();
    let _ = service.publish_state().await;

    let client = Arc::new(VoidInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func_void().await.is_ok());
}
