mod nats_common;
use tb_simple::api::no_properties_interface::NoPropertiesInterfaceTrait;
use tb_simple::implementation::no_properties_interface::NoPropertiesInterface;
use tb_simple::nats::no_properties_interface_client::NoPropertiesInterfaceNatsClient;
use tb_simple::nats::no_properties_interface_service::NoPropertiesInterfaceNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_no_properties_interface_roundtrip() {
    let impl_ = Arc::new(NoPropertiesInterface::default());
    let service = Arc::new(NoPropertiesInterfaceNatsService::new(impl_.clone() as Arc<dyn NoPropertiesInterfaceTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(NoPropertiesInterfaceNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.func_void().await.is_ok());
    assert!(client.func_bool(Default::default()).await.is_ok());
}
