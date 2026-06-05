mod nats_common;
#[allow(unused_imports)]
use tb_names::api::data_structs::*;
use tb_names::api::nam_es::NamEsTrait;
use tb_names::implementation::nam_es::NamEs;
use tb_names::nats::nam_es_client::NamEsNatsClient;
use tb_names::nats::nam_es_service::NamEsNatsService;
use std::sync::Arc;

/// End-to-end round-trip over a real nats-server: a generated service wraps the
/// implementation, a generated client talks to it. Ignored by default; the CI
/// integration job starts a nats-server and runs it with `--ignored`.
#[tokio::test(flavor = "multi_thread")]
#[ignore = "requires a running nats-server (NATS_URL or 127.0.0.1:4222)"]
async fn test_nats_nam_es_roundtrip() {
    let impl_ = Arc::new(NamEs::default());
    let service = Arc::new(NamEsNatsService::new(impl_.clone() as Arc<dyn NamEsTrait>, nats_common::connect().await));
    let _service_sub = service.subscribe();

    let client = Arc::new(NamEsNatsClient::new(nats_common::connect().await));
    let _client_sub = client.subscribe();
    nats_common::settle().await;

    // Operations: NATS request/reply round-trip (the service answers each request).
    assert!(client.some_function(Default::default()).await.is_ok());
    assert!(client.some_function2(Default::default()).await.is_ok());

    // Writable properties: a client set propagates over NATS to the service implementation.
    {
        let test_value: bool = true;
        client.set_switch(test_value);
        assert!(nats_common::wait_until(|| impl_.switch() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_some_property(test_value);
        assert!(nats_common::wait_until(|| impl_.some_property() == test_value).await);
    }
    {
        let test_value: i32 = 1i32;
        client.set_some_poperty2(test_value);
        assert!(nats_common::wait_until(|| impl_.some_poperty2() == test_value).await);
    }
    {
        let test_value: Enum_With_Under_scoresEnum = Default::default();
        client.set_enum_property(test_value);
        assert!(nats_common::wait_until(|| impl_.enum_property() == test_value).await);
    }
}
